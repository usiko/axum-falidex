use jsonschema::Validator;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Schéma JSON (draft-07) du corps de `POST /collection/import`, en fichier séparé — même
/// approche que `import-json-schema.json` côté frontend (`falidex-dashboard`) — plutôt
/// qu'encodé uniquement dans les types Rust ci-dessous : c'est la source canonique du contrat
/// de la requête, que les types Rust reflètent.
const IMPORT_JSON_SCHEMA_STR: &str = include_str!("import-json-schema.json");

static IMPORT_JSON_VALIDATOR: Lazy<Validator> = Lazy::new(|| {
    let schema: Value = serde_json::from_str(IMPORT_JSON_SCHEMA_STR)
        .expect("import-json-schema.json doit être un JSON valide");
    jsonschema::validator_for(&schema)
        .expect("import-json-schema.json doit être un schéma JSON valide")
});

/// Valide le JSON brut d'une requête d'import contre `import-json-schema.json`, avant même de
/// tenter de le désérialiser en `ImportBatchRequest`, pour des messages d'erreur agrégés et
/// lisibles (même principe que la validation AJV côté frontend).
pub fn validate_batch_json(value: &Value) -> Result<(), Vec<String>> {
    let errors: Vec<String> = IMPORT_JSON_VALIDATOR
        .iter_errors(value)
        .map(|e| format!("{}: {}", e.instance_path(), e))
        .collect();

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Format JSON d'un batch d'import (QUE-67), généré manuellement par une IA externe à partir
/// d'un prompt (QUE-71) puis revu/corrigé côté dashboard (QUE-70) avant d'être envoyé ici.
///
/// Miroir du schéma `import-json-schema.json` du frontend (`falidex-dashboard`), à la différence
/// que `linkId`/`newLink`/`name` ne font pas partie de ce que l'IA doit produire : ce sont des
/// informations connues côté dashboard (le "code" sélectionné en base à copier/modifier, ou les
/// données de la nouvelle fiche à créer, et le nom donné à cet import pour la traçabilité QUE-68).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImportOperationType {
    Add,
    Update,
    Remove,
}

/// Distingue les référentiels partagés (impactent potentiellement plusieurs fiches) de `relation`
/// (un `LinkItem`, scope local à la fiche ciblée par le batch).
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ImportEntityType {
    Circulaire,
    Color,
    Filiere,
    Placement,
    Position,
    Symbole,
    Signification,
    SymboleSens,
    SymboleAccessoire,
    Relation,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImportOperation {
    pub op: ImportOperationType,
    pub entity: ImportEntityType,
    /// Id réel si l'entité existe déjà, ou id temporaire préfixé "tmp:" pour une entité créée
    /// plus tôt dans le même batch (résolu en id réel MongoDB au moment de l'application, QUE-69).
    pub id: String,
    /// Champs de l'entité concernée. Pour un `update`, uniquement les champs qui changent.
    /// Pour un `remove`, généralement absent.
    #[serde(default)]
    pub fields: Map<String, Value>,
    pub confidence: f64,
    pub incertain: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Données de la nouvelle fiche à créer quand aucun code existant n'est sélectionné comme base
/// ("Aucun (nouvelle fiche)" côté prompt, QUE-71).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImportNewLinkFields {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annee: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImportBatchRequest {
    /// Nom donné à cet import dans le dashboard, pour la traçabilité (QUE-68).
    pub name: String,
    /// Id du code existant à mettre à jour. Mutuellement exclusif avec `new_link`.
    #[serde(rename = "linkId", skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    /// Données de la fiche à créer si aucun code existant n'est ciblé. Mutuellement exclusif
    /// avec `link_id`.
    #[serde(rename = "newLink", skip_serializing_if = "Option::is_none")]
    pub new_link: Option<ImportNewLinkFields>,
    pub operations: Vec<ImportOperation>,
}

/// Trace d'une opération réellement appliquée (ou tentée) par un import, pour la traçabilité
/// QUE-68 : "avant/après par item".
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImportOperationResult {
    pub op: ImportOperationType,
    pub entity: ImportEntityType,
    /// Id tel que fourni dans le batch (réel ou "tmp:...").
    #[serde(rename = "sourceId")]
    pub source_id: String,
    /// Id réel MongoDB effectivement utilisé, après résolution des ids temporaires.
    #[serde(rename = "resolvedId")]
    pub resolved_id: String,
    /// État avant l'opération (absent pour un `add`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<Value>,
    /// État après l'opération (absent pour un `remove`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Value>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ImportCodeStatus {
    Applied,
    Failed,
}

/// Le "code" (au sens : nom donné à un batch d'import) tracé pour QUE-68. Nouvelle collection,
/// distincte des "codes" au sens fiche (`LinkDetail`) : ici il s'agit du nom donné à *cet import*
/// dans le dashboard, pas d'une fiche de décoration.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImportCode {
    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub date: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    /// Fiche ciblée par l'import (existante ou nouvellement créée), si les opérations en
    /// touchaient une.
    #[serde(rename = "linkId", skip_serializing_if = "Option::is_none")]
    pub link_id: Option<String>,
    pub status: ImportCodeStatus,
    pub operations: Vec<ImportOperationResult>,
    /// Message d'erreur si `status` est `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_the_batch_shape_produced_by_the_dashboard_prompt() {
        let json = r#"{
            "name": "Import circulaire 2024-12",
            "linkId": null,
            "newLink": { "name": "Croix de guerre 1939-1945", "annee": 1939 },
            "operations": [
                {
                    "op": "add",
                    "entity": "symbole",
                    "id": "tmp:symbole-1",
                    "fields": { "name": "Croix de guerre 1939-1945" },
                    "confidence": 0.95,
                    "incertain": false
                },
                {
                    "op": "add",
                    "entity": "relation",
                    "id": "tmp:relation-1",
                    "fields": { "symboleId": "tmp:symbole-1" },
                    "confidence": 0.4,
                    "incertain": true,
                    "note": "Filière non précisée dans le PDF, tableau ambigu."
                }
            ]
        }"#;

        let batch: ImportBatchRequest = serde_json::from_str(json).expect("should deserialize");

        assert_eq!(batch.name, "Import circulaire 2024-12");
        assert!(batch.link_id.is_none());
        assert_eq!(batch.new_link.unwrap().annee, Some(1939));
        assert_eq!(batch.operations.len(), 2);

        let symbole_op = &batch.operations[0];
        assert_eq!(symbole_op.op, ImportOperationType::Add);
        assert_eq!(symbole_op.entity, ImportEntityType::Symbole);
        assert_eq!(symbole_op.id, "tmp:symbole-1");
        assert!(!symbole_op.incertain);

        let relation_op = &batch.operations[1];
        assert_eq!(relation_op.entity, ImportEntityType::Relation);
        assert!(relation_op.incertain);
        assert_eq!(
            relation_op.fields.get("symboleId").and_then(|v| v.as_str()),
            Some("tmp:symbole-1")
        );
        assert_eq!(
            relation_op.note.as_deref(),
            Some("Filière non précisée dans le PDF, tableau ambigu.")
        );
    }

    #[test]
    fn rejects_an_unknown_entity_value() {
        let json = r#"{
            "name": "x",
            "operations": [
                { "op": "add", "entity": "not-a-real-entity", "id": "1", "confidence": 1, "incertain": false }
            ]
        }"#;

        let result: Result<ImportBatchRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn defaults_fields_to_an_empty_map_when_absent() {
        let json = r#"{
            "name": "x",
            "linkId": "abc",
            "operations": [
                { "op": "remove", "entity": "color", "id": "abc-color", "confidence": 1, "incertain": false }
            ]
        }"#;

        let batch: ImportBatchRequest = serde_json::from_str(json).expect("should deserialize");
        assert!(batch.operations[0].fields.is_empty());
    }

    #[test]
    fn round_trips_an_import_code_through_json() {
        let code = ImportCode {
            id: "code-1".to_string(),
            name: "Import circulaire 2024-12".to_string(),
            date: "2026-07-27T00:00:00Z".to_string(),
            user_id: "user-1".to_string(),
            user_name: "quentin".to_string(),
            link_id: Some("link-1".to_string()),
            status: ImportCodeStatus::Applied,
            operations: vec![ImportOperationResult {
                op: ImportOperationType::Add,
                entity: ImportEntityType::Symbole,
                source_id: "tmp:symbole-1".to_string(),
                resolved_id: "real-id-1".to_string(),
                before: None,
                after: Some(serde_json::json!({ "name": "Croix de guerre 1939-1945" })),
            }],
            error: None,
        };

        let json = serde_json::to_string(&code).expect("should serialize");
        let parsed: ImportCode = serde_json::from_str(&json).expect("should deserialize");

        assert_eq!(parsed.id, "code-1");
        assert_eq!(parsed.status, ImportCodeStatus::Applied);
        assert_eq!(parsed.operations.len(), 1);
        assert_eq!(parsed.operations[0].resolved_id, "real-id-1");
        assert!(json.contains("\"_id\":\"code-1\""));
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn validate_batch_json_accepts_a_conforming_batch() {
        let value = serde_json::json!({
            "name": "Import circulaire 2024-12",
            "newLink": { "name": "Croix de guerre 1939-1945" },
            "operations": [
                {
                    "op": "add",
                    "entity": "symbole",
                    "id": "tmp:symbole-1",
                    "fields": { "name": "Croix de guerre 1939-1945" },
                    "confidence": 0.95,
                    "incertain": false
                }
            ]
        });

        assert_eq!(validate_batch_json(&value), Ok(()));
    }

    #[test]
    fn validate_batch_json_rejects_an_incomplete_operation() {
        let value = serde_json::json!({
            "name": "Import circulaire 2024-12",
            "operations": [
                { "op": "add", "entity": "symbole" }
            ]
        });

        let errors = validate_batch_json(&value).expect_err("should be rejected");
        assert!(!errors.is_empty());
    }

    #[test]
    fn validate_batch_json_rejects_an_unknown_top_level_property() {
        let value = serde_json::json!({
            "name": "x",
            "operations": [],
            "unexpectedField": true
        });

        assert!(validate_batch_json(&value).is_err());
    }

    #[test]
    fn validate_batch_json_rejects_a_confidence_out_of_range() {
        let value = serde_json::json!({
            "name": "x",
            "operations": [
                { "op": "add", "entity": "color", "id": "tmp:color-1", "confidence": 1.5, "incertain": false }
            ]
        });

        assert!(validate_batch_json(&value).is_err());
    }
}
