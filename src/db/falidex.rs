use crate::model::falidex_model::*;
use std::fs;
use std::io::Read;

pub async fn get_circulaires() -> Result<Vec<Circulaire>, String> {
    let data = read_file("src/mock/circulaires.json".to_string()).map_err(|e| e.to_string())?;
    let circulaires: Vec<Circulaire> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(circulaires)
}

pub async fn get_colors() -> Result<Vec<Color>, String> {
    let data = read_file("src/mock/colors.json".to_string()).map_err(|e| e.to_string())?;
    let colors: Vec<Color> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(colors)
}

pub async fn get_filieres() -> Result<Vec<Filiere>, String> {
    let data = read_file("src/mock/filieres.json".to_string()).map_err(|e| e.to_string())?;
    let filieres: Vec<Filiere> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(filieres)
}

pub async fn get_placements() -> Result<Vec<Placement>, String> {
    let data = read_file("src/mock/placements.json".to_string()).map_err(|e| e.to_string())?;
    let placements: Vec<Placement> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(placements)
}

pub async fn get_positions() -> Result<Vec<Position>, String> {
    let data = read_file("src/mock/positions.json".to_string()).map_err(|e| e.to_string())?;
    let positions: Vec<Position> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(positions)
}

pub async fn get_significations() -> Result<Vec<Signification>, String> {
    let data = read_file("src/mock/significations.json".to_string()).map_err(|e| e.to_string())?;
    let significations: Vec<Signification> =
        serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(significations)
}

pub async fn get_symbole_accessoires() -> Result<Vec<SymboleAccessoire>, String> {
    let data =
        read_file("src/mock/symbole-accessoire.json".to_string()).map_err(|e| e.to_string())?;
    let symbole_accessoires: Vec<SymboleAccessoire> =
        serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(symbole_accessoires)
}

pub async fn get_symboles_sens() -> Result<Vec<SymboleSens>, String> {
    let data = read_file("src/mock/symboles-sens.json".to_string()).map_err(|e| e.to_string())?;
    let symboles_sens: Vec<SymboleSens> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(symboles_sens)
}

pub async fn get_symboles() -> Result<Vec<Symbole>, String> {
    let data = read_file("src/mock/symboles.json".to_string()).map_err(|e| e.to_string())?;
    let symboles: Vec<Symbole> = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(symboles)
}

pub async fn get_circulaires_colors() -> Result<Vec<CirculaireColor>, String> {
    let data =
        read_file("src/mock/circulaires-colors.json".to_string()).map_err(|e| e.to_string())?;
    let circulaires_colors: Vec<CirculaireColor> =
        serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(circulaires_colors)
}

fn read_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file =
        fs::File::open(&path).map_err(|e| format!("Erreur ouverture fichier '{}': {}", path, e))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}
