use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Circulaire {
    pub matiere: String,
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Color {
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
    pub name: String,
    #[serde(rename = "colorData")]
    pub color_data: String,
}

#[derive(Deserialize, Serialize)]
pub struct Filiere {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Placement {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Position {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Signification {
    pub content: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SymboleAccessoire {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SymboleSens {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Img {
    pub id: String,
    pub url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Symbole {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<Img>>,
}

#[derive(Deserialize, Serialize)]
pub struct CirculaireColor {
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
    #[serde(rename = "circulaireId")]
    pub circulaire_id: String,
    #[serde(rename = "colorIds")]
    pub color_ids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
}

#[derive(Deserialize, Serialize)]
pub struct LinkDetail {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
    pub relations: Vec<LinkItem>,
    pub specificites: Vec<Specificite>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annee: Option<u64>,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ville: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct LinkDetailReq {
    pub name: String,
    pub id: String,
    pub relations: Vec<LinkItem>,
    pub specificites: Vec<Specificite>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ville: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateLinkDetail {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub relations: Vec<LinkItem>,
    pub specificites: Vec<Specificite>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastUpdate", skip_serializing_if = "Option::is_none")]
    pub last_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annee: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ville: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Specificite {
    pub name: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub article: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct LinkItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "placementId")]
    pub placement_id: String,
    #[serde(rename = "positionId", skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    #[serde(rename = "filiereId", skip_serializing_if = "Option::is_none")]
    pub filiere_id: Option<String>,
    #[serde(rename = "symboleId", skip_serializing_if = "Option::is_none")]
    pub symbole_id: Option<String>,
    #[serde(rename = "circulaireId", skip_serializing_if = "Option::is_none")]
    pub circulaire_id: Option<String>,
    #[serde(rename = "significationId", skip_serializing_if = "Option::is_none")]
    pub signification_id: Option<String>,
    #[serde(rename = "symboleSensId", skip_serializing_if = "Option::is_none")]
    pub symbole_sens_id: Option<String>,
    #[serde(rename = "symboleAccessoryId", skip_serializing_if = "Option::is_none")]
    pub symbole_accessory_id: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spe: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blame: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct OccurenceDetail {
    pub relation: String,
    pub items: u64,
}

#[derive(Deserialize, Serialize)]
pub struct Log {
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
    pub date: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub info: String,
}
