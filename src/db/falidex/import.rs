use std::collections::HashMap;

use chrono::Utc;
use mongodb::bson::{Document, doc};
use mongodb::{Client, ClientSession, Collection, Database};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};
use uuid::Uuid;

use super::import_log;
use crate::model::falidex_model::LinkDetail;
use crate::model::import_model::{
    ImportBatchRequest, ImportCode, ImportCodeStatus, ImportEntityType, ImportOperation,
    ImportOperationResult, ImportOperationType,
};

const TMP_PREFIX: &str = "tmp:";

/// Applique un batch d'import déjà revu/validé côté front (QUE-70) : résout les ids temporaires,
/// applique la série d'opérations de façon transactionnelle (tout ou rien), et enregistre un
/// "code" nommé pour la traçabilité (QUE-68). Refuse l'import si un item est encore marqué
/// `incertain` (garde-fou côté back, en plus du blocage déjà fait côté front).
pub async fn apply_batch(
    db: &Database,
    client: &Client,
    user_id: String,
    req: ImportBatchRequest,
) -> Result<ImportCode, String> {
    if let Some(op) = req.operations.iter().find(|o| o.incertain) {
        return Err(format!(
            "Import refusé : une opération sur {:?} (id {}) est encore marquée incertaine",
            op.entity, op.id
        ));
    }

    if req.link_id.is_some() && req.new_link.is_some() {
        return Err("linkId et newLink ne peuvent pas être fournis en même temps".to_string());
    }

    let has_relation_ops = req
        .operations
        .iter()
        .any(|o| o.entity == ImportEntityType::Relation);
    if has_relation_ops && req.link_id.is_none() && req.new_link.is_none() {
        return Err(
            "Un code cible (existant via linkId, ou nouveau via newLink) est requis pour appliquer des opérations de relation"
                .to_string(),
        );
    }

    // Résolution des ids temporaires : uniquement pour les référentiels partagés (add), en une
    // passe indépendante de l'ordre du batch. Les relations n'ont pas besoin d'être résolues ici :
    // leur id temporaire (toujours fourni par le schéma) n'est référencé par aucune autre opération.
    let mut tmp_ids: HashMap<String, String> = HashMap::new();
    for op in &req.operations {
        if op.op == ImportOperationType::Add
            && op.entity != ImportEntityType::Relation
            && op.id.starts_with(TMP_PREFIX)
        {
            tmp_ids.insert(op.id.clone(), Uuid::new_v4().to_string());
        }
    }

    let mut session = client
        .start_session()
        .await
        .map_err(|e| format!("Erreur de session Mongo: {e}"))?;
    session
        .start_transaction()
        .await
        .map_err(|e| format!("Erreur de démarrage de transaction: {e}"))?;

    let outcome = apply_within_transaction(db, &mut session, &req, &tmp_ids).await;

    match &outcome {
        Ok(_) => {
            session
                .commit_transaction()
                .await
                .map_err(|e| format!("Erreur lors du commit de la transaction: {e}"))?;
        }
        Err(_) => {
            // On tente d'annuler proprement, mais on ne masque pas l'erreur d'origine si
            // l'abandon échoue lui-même (connexion perdue, etc.) : c'est de toute façon "tout ou
            // rien" côté données, l'annulation automatique à la clôture de la session couvre ce cas.
            let _ = session.abort_transaction().await;
        }
    }

    let user_name = crate::db::user::get_by_id(db, user_id.clone())
        .await
        .map(|u| u.username)
        .unwrap_or_else(|_| "unknown".to_string());
    let now = Utc::now().to_rfc3339();

    let code = match &outcome {
        Ok((results, link_id)) => ImportCode {
            id: Uuid::new_v4().to_string(),
            name: req.name.clone(),
            date: now,
            user_id,
            user_name,
            link_id: link_id.clone(),
            status: ImportCodeStatus::Applied,
            operations: results.clone(),
            error: None,
        },
        Err(e) => ImportCode {
            id: Uuid::new_v4().to_string(),
            name: req.name.clone(),
            date: now,
            user_id,
            user_name,
            link_id: req.link_id.clone(),
            status: ImportCodeStatus::Failed,
            operations: vec![],
            error: Some(e.clone()),
        },
    };

    // Best-effort : la trace d'import est un historique, son échec ne doit pas masquer le
    // résultat réel de l'application du batch (déjà commité ou abandonné à ce stade).
    if let Err(log_err) = import_log::record(db, &code).await {
        eprintln!("Erreur lors de l'enregistrement de la trace d'import: {log_err}");
    }

    outcome.map(|_| code)
}

async fn apply_within_transaction(
    db: &Database,
    session: &mut ClientSession,
    req: &ImportBatchRequest,
    tmp_ids: &HashMap<String, String>,
) -> Result<(Vec<ImportOperationResult>, Option<String>), String> {
    let mut results = Vec::new();

    // 1. Résoudre (ou créer) la fiche ciblée par les éventuelles opérations de relation.
    let mut target_link: Option<LinkDetail> = if let Some(link_id) = &req.link_id {
        let col: Collection<LinkDetail> = db.collection("links");
        let existing = col
            .find_one(doc! { "_id": link_id })
            .session(&mut *session)
            .await
            .map_err(|e| format!("Erreur lors de la récupération du code cible: {e}"))?;
        Some(existing.ok_or_else(|| format!("Code cible introuvable: {link_id}"))?)
    } else if let Some(new_link) = &req.new_link {
        let now = Utc::now().to_rfc3339();
        Some(LinkDetail {
            id: Uuid::new_v4().to_string(),
            name: new_link.name.clone(),
            relations: vec![],
            specificites: vec![],
            annee: new_link.annee,
            last_update: now.clone(),
            created_at: Some(now),
            default: Some(false),
            visible: Some(true),
            editable: Some(true),
            national: None,
            ville: None,
        })
    } else {
        None
    };
    let is_new_link = req.link_id.is_none();

    // 2. Opérations sur les référentiels partagés (non anodines : impactent potentiellement
    // plusieurs fiches), dans l'ordre du batch.
    for op in &req.operations {
        if op.entity == ImportEntityType::Relation {
            continue;
        }
        let resolved_id = resolve_id(&op.id, tmp_ids)?;
        let (before, after) = apply_referential_operation(db, session, op, &resolved_id).await?;
        results.push(ImportOperationResult {
            op: op.op,
            entity: op.entity,
            source_id: op.id.clone(),
            resolved_id,
            before,
            after,
        });
    }

    // 3. Opérations de relation (scope local à la fiche ciblée), appliquées en mémoire.
    for op in &req.operations {
        if op.entity != ImportEntityType::Relation {
            continue;
        }
        let link = target_link.as_mut().ok_or_else(|| {
            "Aucun code cible (linkId/newLink) pour appliquer les opérations de relation"
                .to_string()
        })?;

        let (resolved_id, before, after) = apply_relation_operation(link, op, tmp_ids)?;
        results.push(ImportOperationResult {
            op: op.op,
            entity: op.entity,
            source_id: op.id.clone(),
            resolved_id,
            before,
            after,
        });
    }

    // 4. Persister la fiche ciblée si elle a été touchée (créée, ou dont les relations ont changé).
    let link_id = if let Some(link) = &mut target_link {
        link.last_update = Utc::now().to_rfc3339();
        let col: Collection<LinkDetail> = db.collection("links");
        if is_new_link {
            col.insert_one(&*link)
                .session(&mut *session)
                .await
                .map_err(|e| format!("Erreur lors de la création du code: {e}"))?;
        } else {
            col.replace_one(doc! { "_id": &link.id }, &*link)
                .session(&mut *session)
                .await
                .map_err(|e| format!("Erreur lors de la mise à jour du code: {e}"))?;
        }
        Some(link.id.clone())
    } else {
        None
    };

    Ok((results, link_id))
}

fn resolve_id(id: &str, tmp_ids: &HashMap<String, String>) -> Result<String, String> {
    if id.starts_with(TMP_PREFIX) {
        tmp_ids
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Id temporaire non résolu: {id}"))
    } else {
        Ok(id.to_string())
    }
}

fn resolve_optional_id(
    id: Option<String>,
    tmp_ids: &HashMap<String, String>,
) -> Result<Option<String>, String> {
    match id {
        None => Ok(None),
        Some(v) if v.starts_with(TMP_PREFIX) => tmp_ids
            .get(&v)
            .cloned()
            .map(Some)
            .ok_or_else(|| format!("Id temporaire non résolu: {v}")),
        Some(v) => Ok(Some(v)),
    }
}

fn field_present(fields: &Map<String, Value>, key: &str) -> bool {
    fields.contains_key(key)
}

fn field_as_optional_string(
    fields: &Map<String, Value>,
    key: &str,
) -> Result<Option<String>, String> {
    match fields.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(format!("le champ '{key}' doit être une chaîne ou null")),
    }
}

fn field_as_optional_bool(fields: &Map<String, Value>, key: &str) -> Result<Option<bool>, String> {
    match fields.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Bool(b)) => Ok(Some(*b)),
        Some(_) => Err(format!("le champ '{key}' doit être un booléen ou null")),
    }
}

/// Vérifie qu'un référentiel partagé n'est encore référencé par aucune fiche avant de le
/// supprimer, comme le font déjà les endpoints CRUD dédiés (ex. `color::delete`).
///
/// Limite connue : `color` n'a pas de champ dédié dans `LinkItem` (il est associé à une
/// circulaire via `circulaires-colors`, hors scope du schéma d'import QUE-67) ; la suppression
/// d'une couleur via l'import n'est donc pas gardée par cette vérification.
async fn check_not_referenced(
    db: &Database,
    session: &mut ClientSession,
    entity: ImportEntityType,
    id: &str,
) -> Result<(), String> {
    let field = match entity {
        ImportEntityType::Filiere => "filiereId",
        ImportEntityType::Symbole => "symboleId",
        ImportEntityType::Placement => "placementId",
        ImportEntityType::Position => "positionId",
        ImportEntityType::Circulaire => "circulaireId",
        ImportEntityType::Signification => "significationId",
        ImportEntityType::SymboleSens => "symboleSensId",
        ImportEntityType::SymboleAccessoire => "symboleAccessoryId",
        ImportEntityType::Color | ImportEntityType::Relation => return Ok(()),
    };

    let mut elem_match = Document::new();
    elem_match.insert(field, id);
    let mut query = Document::new();
    query.insert("relations", doc! { "$elemMatch": elem_match });

    let col: Collection<Document> = db.collection("links");
    let count = col
        .count_documents(query)
        .session(&mut *session)
        .await
        .map_err(|e| e.to_string())?;

    if count > 0 {
        Err(format!(
            "Impossible de supprimer ce {entity:?} ({id}) : encore référencé par {count} fiche(s)"
        ))
    } else {
        Ok(())
    }
}

async fn apply_generic<F: DeserializeOwned>(
    db: &Database,
    session: &mut ClientSession,
    col: &Collection<Document>,
    op: &ImportOperation,
    resolved_id: &str,
    build_insert: impl FnOnce(&F) -> Result<Document, String>,
    build_update: impl FnOnce(&F) -> Document,
) -> Result<(Option<Value>, Option<Value>), String> {
    let fields: F = serde_json::from_value(Value::Object(op.fields.clone()))
        .map_err(|e| format!("Champs invalides pour {:?} {}: {e}", op.entity, op.id))?;

    match op.op {
        ImportOperationType::Add => {
            let insert_doc = build_insert(&fields)?;
            col.insert_one(insert_doc.clone())
                .session(&mut *session)
                .await
                .map_err(|e| format!("Erreur lors de l'ajout: {e}"))?;
            Ok((
                None,
                Some(serde_json::to_value(&insert_doc).unwrap_or(Value::Null)),
            ))
        }
        ImportOperationType::Update => {
            let before = col
                .find_one(doc! { "_id": resolved_id })
                .session(&mut *session)
                .await
                .map_err(|e| e.to_string())?
                .map(|d| serde_json::to_value(&d).unwrap_or(Value::Null));

            let update_doc = build_update(&fields);
            if update_doc.is_empty() {
                return Err(format!(
                    "Aucun champ à mettre à jour pour {:?} {}",
                    op.entity, op.id
                ));
            }

            let result = col
                .update_one(doc! { "_id": resolved_id }, doc! { "$set": update_doc })
                .session(&mut *session)
                .await
                .map_err(|e| format!("Erreur lors de la mise à jour: {e}"))?;
            if result.matched_count == 0 {
                return Err(format!("{:?} introuvable: {}", op.entity, resolved_id));
            }

            let after = col
                .find_one(doc! { "_id": resolved_id })
                .session(&mut *session)
                .await
                .map_err(|e| e.to_string())?
                .map(|d| serde_json::to_value(&d).unwrap_or(Value::Null));
            Ok((before, after))
        }
        ImportOperationType::Remove => {
            check_not_referenced(db, session, op.entity, resolved_id).await?;

            let before = col
                .find_one(doc! { "_id": resolved_id })
                .session(&mut *session)
                .await
                .map_err(|e| e.to_string())?
                .map(|d| serde_json::to_value(&d).unwrap_or(Value::Null));
            if before.is_none() {
                return Err(format!("{:?} introuvable: {}", op.entity, resolved_id));
            }

            col.delete_one(doc! { "_id": resolved_id })
                .session(&mut *session)
                .await
                .map_err(|e| format!("Erreur lors de la suppression: {e}"))?;
            Ok((before, None))
        }
    }
}

#[derive(serde::Deserialize)]
struct NameFields {
    name: Option<String>,
}

#[derive(serde::Deserialize)]
struct CirculaireFields {
    matiere: Option<String>,
    name: Option<String>,
}

#[derive(serde::Deserialize)]
struct ColorFields {
    name: Option<String>,
    #[serde(rename = "colorData")]
    color_data: Option<String>,
}

#[derive(serde::Deserialize)]
struct SignificationFields {
    content: Option<String>,
}

async fn simple_name_entity(
    db: &Database,
    session: &mut ClientSession,
    collection_name: &str,
    op: &ImportOperation,
    resolved_id: &str,
) -> Result<(Option<Value>, Option<Value>), String> {
    let col: Collection<Document> = db.collection(collection_name);
    apply_generic::<NameFields>(
        db,
        session,
        &col,
        op,
        resolved_id,
        |f| {
            Ok(doc! {
                "_id": resolved_id,
                "name": f.name.clone().ok_or_else(|| "le champ 'name' est requis".to_string())?,
            })
        },
        |f| {
            let mut d = doc! {};
            if let Some(v) = &f.name {
                d.insert("name", v.clone());
            }
            d
        },
    )
    .await
}

async fn apply_referential_operation(
    db: &Database,
    session: &mut ClientSession,
    op: &ImportOperation,
    resolved_id: &str,
) -> Result<(Option<Value>, Option<Value>), String> {
    match op.entity {
        ImportEntityType::Circulaire => {
            let col: Collection<Document> = db.collection("circulaires");
            apply_generic::<CirculaireFields>(
                db,
                session,
                &col,
                op,
                resolved_id,
                |f| {
                    Ok(doc! {
                        "_id": resolved_id,
                        "matiere": f.matiere.clone().ok_or_else(|| "le champ 'matiere' est requis".to_string())?,
                        "name": f.name.clone().ok_or_else(|| "le champ 'name' est requis".to_string())?,
                    })
                },
                |f| {
                    let mut d = doc! {};
                    if let Some(v) = &f.matiere {
                        d.insert("matiere", v.clone());
                    }
                    if let Some(v) = &f.name {
                        d.insert("name", v.clone());
                    }
                    d
                },
            )
            .await
        }
        ImportEntityType::Color => {
            let col: Collection<Document> = db.collection("colors");
            apply_generic::<ColorFields>(
                db,
                session,
                &col,
                op,
                resolved_id,
                |f| {
                    Ok(doc! {
                        "_id": resolved_id,
                        "name": f.name.clone().ok_or_else(|| "le champ 'name' est requis".to_string())?,
                        "colorData": f.color_data.clone().ok_or_else(|| "le champ 'colorData' est requis".to_string())?,
                    })
                },
                |f| {
                    let mut d = doc! {};
                    if let Some(v) = &f.name {
                        d.insert("name", v.clone());
                    }
                    if let Some(v) = &f.color_data {
                        d.insert("colorData", v.clone());
                    }
                    d
                },
            )
            .await
        }
        ImportEntityType::Signification => {
            let col: Collection<Document> = db.collection("significations");
            apply_generic::<SignificationFields>(
                db,
                session,
                &col,
                op,
                resolved_id,
                |f| {
                    Ok(doc! {
                        "_id": resolved_id,
                        "content": f.content.clone().ok_or_else(|| "le champ 'content' est requis".to_string())?,
                    })
                },
                |f| {
                    let mut d = doc! {};
                    if let Some(v) = &f.content {
                        d.insert("content", v.clone());
                    }
                    d
                },
            )
            .await
        }
        ImportEntityType::Filiere => {
            simple_name_entity(db, session, "filieres", op, resolved_id).await
        }
        ImportEntityType::Placement => {
            simple_name_entity(db, session, "placements", op, resolved_id).await
        }
        ImportEntityType::Position => {
            simple_name_entity(db, session, "positions", op, resolved_id).await
        }
        ImportEntityType::Symbole => {
            simple_name_entity(db, session, "symboles", op, resolved_id).await
        }
        ImportEntityType::SymboleSens => {
            simple_name_entity(db, session, "symboles-sens", op, resolved_id).await
        }
        ImportEntityType::SymboleAccessoire => {
            simple_name_entity(db, session, "symboles-accessories", op, resolved_id).await
        }
        ImportEntityType::Relation => unreachable!("les relations sont appliquées séparément"),
    }
}

fn build_link_item_from_fields(
    fields_json: &Map<String, Value>,
    tmp_ids: &HashMap<String, String>,
    existing: Option<&crate::model::falidex_model::LinkItem>,
) -> Result<crate::model::falidex_model::LinkItem, String> {
    use crate::model::falidex_model::LinkItem;

    let now = Utc::now().to_rfc3339();

    let resolve_field =
        |key: &str, existing_value: Option<String>| -> Result<Option<String>, String> {
            if field_present(fields_json, key) {
                resolve_optional_id(field_as_optional_string(fields_json, key)?, tmp_ids)
            } else {
                Ok(existing_value)
            }
        };
    let resolve_bool_field =
        |key: &str, existing_value: Option<bool>| -> Result<Option<bool>, String> {
            if field_present(fields_json, key) {
                field_as_optional_bool(fields_json, key)
            } else {
                Ok(existing_value)
            }
        };

    let placement_id = resolve_field("placementId", existing.map(|e| e.placement_id.clone()))?
        .ok_or_else(|| "le champ 'placementId' est requis pour une relation".to_string())?;

    Ok(LinkItem {
        id: Some(
            existing
                .and_then(|e| e.id.clone())
                .unwrap_or_else(|| Uuid::new_v4().to_string()),
        ),
        placement_id,
        position_id: resolve_field("positionId", existing.and_then(|e| e.position_id.clone()))?,
        filiere_id: resolve_field("filiereId", existing.and_then(|e| e.filiere_id.clone()))?,
        symbole_id: resolve_field("symboleId", existing.and_then(|e| e.symbole_id.clone()))?,
        circulaire_id: resolve_field(
            "circulaireId",
            existing.and_then(|e| e.circulaire_id.clone()),
        )?,
        signification_id: resolve_field(
            "significationId",
            existing.and_then(|e| e.signification_id.clone()),
        )?,
        symbole_sens_id: resolve_field(
            "symboleSensId",
            existing.and_then(|e| e.symbole_sens_id.clone()),
        )?,
        symbole_accessory_id: resolve_field(
            "symboleAccessoryId",
            existing.and_then(|e| e.symbole_accessory_id.clone()),
        )?,
        created_at: existing
            .and_then(|e| e.created_at.clone())
            .or_else(|| Some(now.clone())),
        updated_at: Some(now),
        spe: resolve_bool_field("spe", existing.and_then(|e| e.spe))?,
        absent: resolve_bool_field("absent", existing.and_then(|e| e.absent))?,
        blame: resolve_bool_field("blame", existing.and_then(|e| e.blame))?,
    })
}

fn apply_relation_operation(
    link: &mut LinkDetail,
    op: &ImportOperation,
    tmp_ids: &HashMap<String, String>,
) -> Result<(String, Option<Value>, Option<Value>), String> {
    match op.op {
        ImportOperationType::Add => {
            let item = build_link_item_from_fields(&op.fields, tmp_ids, None)?;
            let resolved_id = item.id.clone().unwrap_or_default();
            let after = serde_json::to_value(&item).map_err(|e| e.to_string())?;
            link.relations.push(item);
            Ok((resolved_id, None, Some(after)))
        }
        ImportOperationType::Update => {
            let pos = link
                .relations
                .iter()
                .position(|r| r.id.as_deref() == Some(op.id.as_str()))
                .ok_or_else(|| format!("Relation introuvable: {}", op.id))?;
            let before = serde_json::to_value(&link.relations[pos]).map_err(|e| e.to_string())?;
            let updated =
                build_link_item_from_fields(&op.fields, tmp_ids, Some(&link.relations[pos]))?;
            let after = serde_json::to_value(&updated).map_err(|e| e.to_string())?;
            link.relations[pos] = updated;
            Ok((op.id.clone(), Some(before), Some(after)))
        }
        ImportOperationType::Remove => {
            let pos = link
                .relations
                .iter()
                .position(|r| r.id.as_deref() == Some(op.id.as_str()))
                .ok_or_else(|| format!("Relation introuvable: {}", op.id))?;
            let before = serde_json::to_value(&link.relations[pos]).map_err(|e| e.to_string())?;
            link.relations.remove(pos);
            Ok((op.id.clone(), Some(before), None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::falidex_model::{LinkItem, Specificite};
    use serde_json::json;

    fn operation(
        op: ImportOperationType,
        entity: ImportEntityType,
        id: &str,
        fields: Value,
    ) -> ImportOperation {
        ImportOperation {
            op,
            entity,
            id: id.to_string(),
            fields: fields.as_object().cloned().unwrap_or_default(),
            confidence: 0.9,
            incertain: false,
            note: None,
        }
    }

    fn empty_link() -> LinkDetail {
        LinkDetail {
            id: "link-1".to_string(),
            name: "Code de test".to_string(),
            relations: vec![],
            specificites: Vec::<Specificite>::new(),
            annee: None,
            last_update: "now".to_string(),
            created_at: None,
            default: None,
            visible: None,
            editable: None,
            national: None,
            ville: None,
        }
    }

    #[test]
    fn resolve_id_passes_through_real_ids() {
        let tmp_ids = HashMap::new();
        assert_eq!(resolve_id("real-id", &tmp_ids).unwrap(), "real-id");
    }

    #[test]
    fn resolve_id_resolves_a_known_tmp_id() {
        let mut tmp_ids = HashMap::new();
        tmp_ids.insert("tmp:symbole-1".to_string(), "real-symbole-1".to_string());
        assert_eq!(
            resolve_id("tmp:symbole-1", &tmp_ids).unwrap(),
            "real-symbole-1"
        );
    }

    #[test]
    fn resolve_id_errors_on_an_unknown_tmp_id() {
        let tmp_ids = HashMap::new();
        assert!(resolve_id("tmp:unknown", &tmp_ids).is_err());
    }

    #[test]
    fn add_relation_resolves_a_tmp_symbole_id_created_earlier_in_the_batch() {
        let mut tmp_ids = HashMap::new();
        tmp_ids.insert("tmp:symbole-1".to_string(), "real-symbole-1".to_string());
        let mut link = empty_link();

        let op = operation(
            ImportOperationType::Add,
            ImportEntityType::Relation,
            "tmp:relation-1",
            json!({ "placementId": "placement-1", "symboleId": "tmp:symbole-1" }),
        );

        let (resolved_id, before, after) =
            apply_relation_operation(&mut link, &op, &tmp_ids).unwrap();

        assert!(before.is_none());
        assert!(after.is_some());
        assert_eq!(link.relations.len(), 1);
        assert_eq!(
            link.relations[0].symbole_id.as_deref(),
            Some("real-symbole-1")
        );
        assert_eq!(link.relations[0].placement_id, "placement-1");
        // L'id de relation généré ne doit jamais être le placeholder "tmp:" du batch.
        assert_ne!(resolved_id, "tmp:relation-1");
        assert_eq!(link.relations[0].id.as_deref(), Some(resolved_id.as_str()));
    }

    #[test]
    fn add_relation_without_placement_id_is_rejected() {
        let tmp_ids = HashMap::new();
        let mut link = empty_link();
        let op = operation(
            ImportOperationType::Add,
            ImportEntityType::Relation,
            "tmp:relation-1",
            json!({ "symboleId": "symbole-1" }),
        );

        assert!(apply_relation_operation(&mut link, &op, &tmp_ids).is_err());
    }

    #[test]
    fn update_relation_only_touches_fields_present_in_the_payload() {
        let tmp_ids = HashMap::new();
        let mut link = empty_link();
        link.relations.push(LinkItem {
            id: Some("relation-1".to_string()),
            placement_id: "placement-1".to_string(),
            position_id: Some("position-1".to_string()),
            filiere_id: Some("filiere-1".to_string()),
            symbole_id: None,
            circulaire_id: None,
            signification_id: None,
            symbole_sens_id: None,
            symbole_accessory_id: None,
            created_at: Some("2020-01-01T00:00:00Z".to_string()),
            updated_at: Some("2020-01-01T00:00:00Z".to_string()),
            spe: None,
            absent: None,
            blame: None,
        });

        // Ne mentionne que symboleId : filiereId/positionId ne doivent pas être touchés.
        let op = operation(
            ImportOperationType::Update,
            ImportEntityType::Relation,
            "relation-1",
            json!({ "symboleId": "symbole-2" }),
        );

        let (resolved_id, before, _after) =
            apply_relation_operation(&mut link, &op, &tmp_ids).unwrap();

        assert_eq!(resolved_id, "relation-1");
        assert!(before.is_some());
        let updated = &link.relations[0];
        assert_eq!(updated.symbole_id.as_deref(), Some("symbole-2"));
        assert_eq!(updated.filiere_id.as_deref(), Some("filiere-1"));
        assert_eq!(updated.position_id.as_deref(), Some("position-1"));
        assert_eq!(updated.placement_id, "placement-1");
        assert_eq!(updated.created_at.as_deref(), Some("2020-01-01T00:00:00Z"));
    }

    #[test]
    fn update_relation_can_explicitly_clear_a_field_with_null() {
        let tmp_ids = HashMap::new();
        let mut link = empty_link();
        link.relations.push(LinkItem {
            id: Some("relation-1".to_string()),
            placement_id: "placement-1".to_string(),
            position_id: Some("position-1".to_string()),
            filiere_id: None,
            symbole_id: None,
            circulaire_id: None,
            signification_id: None,
            symbole_sens_id: None,
            symbole_accessory_id: None,
            created_at: None,
            updated_at: None,
            spe: None,
            absent: None,
            blame: None,
        });

        let op = operation(
            ImportOperationType::Update,
            ImportEntityType::Relation,
            "relation-1",
            json!({ "positionId": null }),
        );

        apply_relation_operation(&mut link, &op, &tmp_ids).unwrap();
        assert!(link.relations[0].position_id.is_none());
    }

    #[test]
    fn update_relation_rejects_an_unknown_relation_id() {
        let tmp_ids = HashMap::new();
        let mut link = empty_link();
        let op = operation(
            ImportOperationType::Update,
            ImportEntityType::Relation,
            "does-not-exist",
            json!({ "symboleId": "symbole-1" }),
        );

        assert!(apply_relation_operation(&mut link, &op, &tmp_ids).is_err());
    }

    #[test]
    fn remove_relation_deletes_the_matching_item() {
        let tmp_ids = HashMap::new();
        let mut link = empty_link();
        link.relations.push(LinkItem {
            id: Some("relation-1".to_string()),
            placement_id: "placement-1".to_string(),
            position_id: None,
            filiere_id: None,
            symbole_id: None,
            circulaire_id: None,
            signification_id: None,
            symbole_sens_id: None,
            symbole_accessory_id: None,
            created_at: None,
            updated_at: None,
            spe: None,
            absent: None,
            blame: None,
        });

        let op = operation(
            ImportOperationType::Remove,
            ImportEntityType::Relation,
            "relation-1",
            json!({}),
        );

        let (resolved_id, before, after) =
            apply_relation_operation(&mut link, &op, &tmp_ids).unwrap();
        assert_eq!(resolved_id, "relation-1");
        assert!(before.is_some());
        assert!(after.is_none());
        assert!(link.relations.is_empty());
    }

    #[test]
    fn field_helpers_distinguish_absent_null_and_present() {
        let fields = json!({ "name": "x", "explicit_null": null })
            .as_object()
            .unwrap()
            .clone();

        assert!(field_present(&fields, "name"));
        assert!(field_present(&fields, "explicit_null"));
        assert!(!field_present(&fields, "missing"));

        assert_eq!(
            field_as_optional_string(&fields, "name").unwrap(),
            Some("x".to_string())
        );
        assert_eq!(
            field_as_optional_string(&fields, "explicit_null").unwrap(),
            None
        );
        assert_eq!(field_as_optional_string(&fields, "missing").unwrap(), None);
    }
}
