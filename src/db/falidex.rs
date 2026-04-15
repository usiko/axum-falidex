use crate::model::falidex_model::Circulaire;
use std::fs;
use std::io::Read;

pub async fn get_circulaires() -> Result<Vec<Circulaire>, String> {
    let data = read_file("src/mock/circulaires.json".to_string()).map_err(|e| e.to_string())?;
    let circulaires: Vec<Circulaire> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(circulaires)
}

fn read_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file =
        fs::File::open(&path).map_err(|e| format!("Erreur ouverture fichier '{}': {}", path, e))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}
