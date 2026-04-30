use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Circulaire {
    matiere: String,
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Color {
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
    name: String,
    #[serde(rename = "colorData")]
    color_data: String,
}

#[derive(Deserialize, Serialize)]
pub struct Filiere {
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Placement {
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Position {
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Signification {
    content: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SymboleAccessoire {
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SymboleSens {
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Img {
    id: String,
    url: String,
}

#[derive(Deserialize, Serialize)]
pub struct Symbole {
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    imgs: Option<Vec<Img>>,
}

#[derive(Deserialize, Serialize)]
pub struct CirculaireColor {
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
    #[serde(rename = "circulaireId")]
    circulaire_id: String,
    #[serde(rename = "colorIds")]
    color_ids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    id: String,
    #[serde(rename = "lastUpdate")]
    last_update: String,
}

#[derive(Deserialize, Serialize)]
pub struct LinkDetail {
    pub name: String,
    #[serde(rename(deserialize = "_id", serialize = "id"))]
    pub id: String,
    pub relations: Vec<LinkItem>,
    pub specificites: Vec<Specificite>,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateLinkDetail {
    pub name: String,
    pub relations: Vec<LinkItem>,
    pub specificites: Vec<Specificite>,
    #[serde(rename = "lastUpdate")]
    pub last_update: String,
}

#[derive(Deserialize, Serialize)]
pub struct Specificite {
    name: String,
    text: String,
    id: String,
    article: String,
}

#[derive(Deserialize, Serialize)]
pub struct LinkItem {
    #[serde(
        rename(deserialize = "_id", serialize = "id"),
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<String>,
    #[serde(rename = "placementId")]
    placement_id: String,
    #[serde(rename = "positionId", skip_serializing_if = "Option::is_none")]
    position_id: Option<String>,
    #[serde(rename = "filiereId", skip_serializing_if = "Option::is_none")]
    filiere_id: Option<String>,
    #[serde(rename = "symboleId", skip_serializing_if = "Option::is_none")]
    symbole_id: Option<String>,
    #[serde(rename = "circulaireId", skip_serializing_if = "Option::is_none")]
    circulaire_id: Option<String>,
    #[serde(rename = "significationId", skip_serializing_if = "Option::is_none")]
    signification_id: Option<String>,
    #[serde(rename = "symboleSensId", skip_serializing_if = "Option::is_none")]
    symbole_sens_id: Option<String>,
    #[serde(rename = "symboleAccessoryId", skip_serializing_if = "Option::is_none")]
    symbole_accessory_id: Option<String>,
}
