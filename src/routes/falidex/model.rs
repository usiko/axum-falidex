use crate::model::falidex_model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Modèles de route pour les requêtes API (utilisent 'id' au lieu de '_id')

#[derive(Deserialize, Serialize)]
pub struct CirculaireCreateReq {
    pub matiere: String,
    pub name: String,
}

impl From<CirculaireCreateReq> for falidex_model::CreateCirculaire {
    fn from(req: CirculaireCreateReq) -> Self {
        falidex_model::CreateCirculaire {
            id: Uuid::new_v4().to_string(),
            matiere: req.matiere,
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CirculaireUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matiere: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<CirculaireUpdateReq> for falidex_model::UpdateCirculaire {
    fn from(req: CirculaireUpdateReq) -> Self {
        falidex_model::UpdateCirculaire {
            matiere: req.matiere,
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ColorCreateReq {
    pub name: String,
    #[serde(rename = "colorData")]
    pub color_data: String,
}

impl From<ColorCreateReq> for falidex_model::CreateColor {
    fn from(req: ColorCreateReq) -> Self {
        falidex_model::CreateColor {
            id: Uuid::new_v4().to_string(),
            name: req.name,
            color_data: req.color_data,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ColorUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "colorData", skip_serializing_if = "Option::is_none")]
    pub color_data: Option<String>,
}

impl From<ColorUpdateReq> for falidex_model::UpdateColor {
    fn from(req: ColorUpdateReq) -> Self {
        falidex_model::UpdateColor {
            name: req.name,
            color_data: req.color_data,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FiliereCreateReq {
    pub name: String,
}

impl From<FiliereCreateReq> for falidex_model::CreateFiliere {
    fn from(req: FiliereCreateReq) -> Self {
        falidex_model::CreateFiliere {
            id: Uuid::new_v4().to_string(),
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct FiliereUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<FiliereUpdateReq> for falidex_model::UpdateFiliere {
    fn from(req: FiliereUpdateReq) -> Self {
        falidex_model::UpdateFiliere { name: req.name }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlacementCreateReq {
    pub name: String,
}

impl From<PlacementCreateReq> for falidex_model::CreatePlacement {
    fn from(req: PlacementCreateReq) -> Self {
        falidex_model::CreatePlacement {
            id: Uuid::new_v4().to_string(),
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PlacementUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<PlacementUpdateReq> for falidex_model::UpdatePlacement {
    fn from(req: PlacementUpdateReq) -> Self {
        falidex_model::UpdatePlacement { name: req.name }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PositionCreateReq {
    pub name: String,
}

impl From<PositionCreateReq> for falidex_model::CreatePosition {
    fn from(req: PositionCreateReq) -> Self {
        falidex_model::CreatePosition {
            id: Uuid::new_v4().to_string(),
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct PositionUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<PositionUpdateReq> for falidex_model::UpdatePosition {
    fn from(req: PositionUpdateReq) -> Self {
        falidex_model::UpdatePosition { name: req.name }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignificationCreateReq {
    pub content: String,
}

impl From<SignificationCreateReq> for falidex_model::CreateSignification {
    fn from(req: SignificationCreateReq) -> Self {
        falidex_model::CreateSignification {
            id: Uuid::new_v4().to_string(),
            content: req.content,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SignificationUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl From<SignificationUpdateReq> for falidex_model::UpdateSignification {
    fn from(req: SignificationUpdateReq) -> Self {
        falidex_model::UpdateSignification {
            content: req.content,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleAccessoireCreateReq {
    pub name: String,
}

impl From<SymboleAccessoireCreateReq> for falidex_model::CreateSymboleAccessoire {
    fn from(req: SymboleAccessoireCreateReq) -> Self {
        falidex_model::CreateSymboleAccessoire {
            id: Uuid::new_v4().to_string(),
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleAccessoireUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<SymboleAccessoireUpdateReq> for falidex_model::UpdateSymboleAccessoire {
    fn from(req: SymboleAccessoireUpdateReq) -> Self {
        falidex_model::UpdateSymboleAccessoire { name: req.name }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleSensCreateReq {
    pub name: String,
}

impl From<SymboleSensCreateReq> for falidex_model::CreateSymboleSens {
    fn from(req: SymboleSensCreateReq) -> Self {
        falidex_model::CreateSymboleSens {
            id: Uuid::new_v4().to_string(),
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleSensUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl From<SymboleSensUpdateReq> for falidex_model::UpdateSymboleSens {
    fn from(req: SymboleSensUpdateReq) -> Self {
        falidex_model::UpdateSymboleSens { name: req.name }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleCreateReq {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<falidex_model::Img>>,
}

impl From<SymboleCreateReq> for falidex_model::CreateSymbole {
    fn from(req: SymboleCreateReq) -> Self {
        falidex_model::CreateSymbole {
            id: Uuid::new_v4().to_string(),
            name: req.name,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct SymboleUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<falidex_model::Img>>,
}

impl From<SymboleUpdateReq> for falidex_model::UpdateSymbole {
    fn from(req: SymboleUpdateReq) -> Self {
        falidex_model::UpdateSymbole {
            name: req.name,
            imgs: req.imgs,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CirculaireColorCreateReq {
    pub circulaire_id: String,
    #[serde(rename = "colorIds")]
    pub color_ids: Vec<String>,
}

impl From<CirculaireColorCreateReq> for falidex_model::CreateCirculaireColor {
    fn from(req: CirculaireColorCreateReq) -> Self {
        falidex_model::CreateCirculaireColor {
            id: Uuid::new_v4().to_string(),
            circulaire_id: req.circulaire_id,
            color_ids: req.color_ids,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CirculaireColorUpdateReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "circulaireId")]
    pub circulaire_id: Option<String>,
    #[serde(rename = "colorIds", skip_serializing_if = "Option::is_none")]
    pub color_ids: Option<Vec<String>>,
}

impl From<CirculaireColorUpdateReq> for falidex_model::UpdateCirculaireColor {
    fn from(req: CirculaireColorUpdateReq) -> Self {
        falidex_model::UpdateCirculaireColor {
            circulaire_id: req.circulaire_id,
            color_ids: req.color_ids,
        }
    }
}
