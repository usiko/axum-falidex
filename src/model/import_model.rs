use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
}
