use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Circulaire {
    matiere: String,
    name: String,
    id: String,
}
