use crate::model::falidex_model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

impl From<CirculaireReq> for falidex_model::CreateCirculaire {
    fn from(req: CirculaireReq) -> Self {
        falidex_model::CreateCirculaire {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            matiere: req.matiere,
            name: req.name,
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

impl From<ColorReq> for falidex_model::CreateColor {
    fn from(req: ColorReq) -> Self {
        falidex_model::CreateColor {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
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

impl From<FiliereReq> for falidex_model::CreateFiliere {
    fn from(req: FiliereReq) -> Self {
        falidex_model::CreateFiliere {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            name: req.name,
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

impl From<PlacementReq> for falidex_model::CreatePlacement {
    fn from(req: PlacementReq) -> Self {
        falidex_model::CreatePlacement {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            name: req.name,
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

impl From<PositionReq> for falidex_model::CreatePosition {
    fn from(req: PositionReq) -> Self {
        falidex_model::CreatePosition {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            name: req.name,
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

impl From<SignificationReq> for falidex_model::CreateSignification {
    fn from(req: SignificationReq) -> Self {
        falidex_model::CreateSignification {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            content: req.content,
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

impl From<SymboleAccessoireReq> for falidex_model::CreateSymboleAccessoire {
    fn from(req: SymboleAccessoireReq) -> Self {
        falidex_model::CreateSymboleAccessoire {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            name: req.name,
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

impl From<SymboleSensReq> for falidex_model::CreateSymboleSens {
    fn from(req: SymboleSensReq) -> Self {
        falidex_model::CreateSymboleSens {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            name: req.name,
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

impl From<SymboleReq> for falidex_model::CreateSymbole {
    fn from(req: SymboleReq) -> Self {
        falidex_model::CreateSymbole {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            name: req.name,
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

impl From<CirculaireColorReq> for falidex_model::CreateCirculaireColor {
    fn from(req: CirculaireColorReq) -> Self {
        falidex_model::CreateCirculaireColor {
            id: req.id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            circulaire_id: req.circulaire_id,
            color_ids: req.color_ids,
        }
    }
}
