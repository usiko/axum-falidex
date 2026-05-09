use crate::model::falidex_model;
use serde::{Deserialize, Serialize};

// Modèles de route pour les requêtes API (utilisent 'id' au lieu de '_id')

#[derive(Deserialize, Serialize)]
pub struct CirculaireReq {
    pub matiere: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl From<CirculaireReq> for falidex_model::Circulaire {
    fn from(req: CirculaireReq) -> Self {
        falidex_model::Circulaire {
            matiere: req.matiere,
            name: req.name,
            id: req.id.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ColorReq {
    pub id: Option<String>,
    pub name: String,
    #[serde(rename = "colorData")]
    pub color_data: String,
}

impl From<ColorReq> for falidex_model::Color {
    fn from(req: ColorReq) -> Self {
        falidex_model::Color {
            id: req.id.unwrap_or_default(),
            name: req.name,
            color_data: req.color_data,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FiliereReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl From<FiliereReq> for falidex_model::Filiere {
    fn from(req: FiliereReq) -> Self {
        falidex_model::Filiere {
            name: req.name,
            id: req.id.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlacementReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl From<PlacementReq> for falidex_model::Placement {
    fn from(req: PlacementReq) -> Self {
        falidex_model::Placement {
            name: req.name,
            id: req.id.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PositionReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl From<PositionReq> for falidex_model::Position {
    fn from(req: PositionReq) -> Self {
        falidex_model::Position {
            name: req.name,
            id: req.id.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignificationReq {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl From<SignificationReq> for falidex_model::Signification {
    fn from(req: SignificationReq) -> Self {
        falidex_model::Signification {
            content: req.content,
            id: req.id.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleAccessoireReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl From<SymboleAccessoireReq> for falidex_model::SymboleAccessoire {
    fn from(req: SymboleAccessoireReq) -> Self {
        falidex_model::SymboleAccessoire {
            name: req.name,
            id: req.id.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleSensReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl From<SymboleSensReq> for falidex_model::SymboleSens {
    fn from(req: SymboleSensReq) -> Self {
        falidex_model::SymboleSens {
            name: req.name,
            id: req.id.unwrap_or_default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<falidex_model::Img>>,
}

impl From<SymboleReq> for falidex_model::Symbole {
    fn from(req: SymboleReq) -> Self {
        falidex_model::Symbole {
            name: req.name,
            id: req.id.unwrap_or_default(),
            imgs: req.imgs,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CirculaireColorReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "circulaireId")]
    pub circulaire_id: String,
    #[serde(rename = "colorIds")]
    pub color_ids: Vec<String>,
}

impl From<CirculaireColorReq> for falidex_model::CirculaireColor {
    fn from(req: CirculaireColorReq) -> Self {
        falidex_model::CirculaireColor {
            id: req.id.unwrap_or_default(),
            circulaire_id: req.circulaire_id,
            color_ids: req.color_ids,
        }
    }
}
