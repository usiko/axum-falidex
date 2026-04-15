use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Circulaire {
    matiere: String,
    name: String,
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Color {
    id: String,
    name: String,
    #[serde(rename = "colorData")]
    color_data: String,
}

#[derive(Deserialize, Serialize)]
pub struct Filiere {
    name: String,
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Placement {
    name: String,
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Position {
    name: String,
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct Signification {
    content: String,
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SymboleAccessoire {
    name: String,
    id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SymboleSens {
    name: String,
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
    id: String,
    imgs: Vec<Img>,
}

#[derive(Deserialize, Serialize)]
pub struct CirculaireColor {
    id: String,
    #[serde(rename = "circulaireId")]
    circulaire_id: String,
    #[serde(rename = "colorIds")]
    color_ids: Vec<String>,
}
