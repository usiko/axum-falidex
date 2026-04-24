use crate::model::falidex_model::*;
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database, bson::doc};
use std::fs;
use std::io::Read;
pub async fn get_circulaires(db: &Database) -> Result<Vec<Circulaire>, String> {
    let col: Collection<Circulaire> = db.collection::<Circulaire>("circulaires");
    let circulaires: Vec<Circulaire> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/circulaires.json".to_string()).map_err(|e| e.to_string())?;
    let circulaires: Vec<Circulaire> = serde_json::from_str(&data).map_err(|e| e.to_string())?;*/

    Ok(circulaires)
}

pub async fn get_colors(db: &Database) -> Result<Vec<Color>, String> {
    let col: Collection<Color> = db.collection::<Color>("colors");
    let colors: Vec<Color> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/colors.json".to_string()).map_err(|e| e.to_string())?;
    let colors: Vec<Color> = serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(colors)
}

pub async fn get_filieres(db: &Database) -> Result<Vec<Filiere>, String> {
    let col: Collection<Filiere> = db.collection::<Filiere>("filieres");
    let filieres: Vec<Filiere> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/filieres.json".to_string()).map_err(|e| e.to_string())?;
    let filieres: Vec<Filiere> = serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(filieres)
}

pub async fn get_placements(db: &Database) -> Result<Vec<Placement>, String> {
    let col: Collection<Placement> = db.collection::<Placement>("placements");
    let placements: Vec<Placement> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/placements.json".to_string()).map_err(|e| e.to_string())?;
    let placements: Vec<Placement> = serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(placements)
}

pub async fn get_positions(db: &Database) -> Result<Vec<Position>, String> {
    let col: Collection<Position> = db.collection::<Position>("positions");
    let positions: Vec<Position> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/positions.json".to_string()).map_err(|e| e.to_string())?;
    let positions: Vec<Position> = serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(positions)
}

pub async fn get_significations(db: &Database) -> Result<Vec<Signification>, String> {
    let col: Collection<Signification> = db.collection::<Signification>("significations");
    let significations: Vec<Signification> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/significations.json".to_string()).map_err(|e| e.to_string())?;
    let significations: Vec<Signification> =
        serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(significations)
}

pub async fn get_symbole_accessoires(db: &Database) -> Result<Vec<SymboleAccessoire>, String> {
    let col: Collection<SymboleAccessoire> =
        db.collection::<SymboleAccessoire>("symboles-accessories");
    let symbole_accessoires: Vec<SymboleAccessoire> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data =
        read_file("src/mock/symbole-accessoire.json".to_string()).map_err(|e| e.to_string())?;
    let symbole_accessoires: Vec<SymboleAccessoire> =
        serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(symbole_accessoires)
}

pub async fn get_symboles_sens(db: &Database) -> Result<Vec<SymboleSens>, String> {
    let col: Collection<SymboleSens> = db.collection::<SymboleSens>("symboles-sens");
    let symboles_sens: Vec<SymboleSens> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/symboles-sens.json".to_string()).map_err(|e| e.to_string())?;
    let symboles_sens: Vec<SymboleSens> = serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(symboles_sens)
}

pub async fn get_symboles(db: &Database) -> Result<Vec<Symbole>, String> {
    let col: Collection<Symbole> = db.collection::<Symbole>("symboles");
    let symboles: Vec<Symbole> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    /*let data = read_file("src/mock/symboles.json".to_string()).map_err(|e| e.to_string())?;
    let symboles: Vec<Symbole> = serde_json::from_str(&data).map_err(|e| e.to_string())?;*/
    Ok(symboles)
}

pub async fn get_circulaires_colors(db: &Database) -> Result<Vec<CirculaireColor>, String> {
    let col: Collection<CirculaireColor> = db.collection::<CirculaireColor>("circulaires-colors");
    let circulaires_colors: Vec<CirculaireColor> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(circulaires_colors)
}

pub async fn get_links(db: &Database) -> Result<Vec<Link>, String> {
    let col: Collection<Link> = db.collection::<Link>("links");
    let data: Vec<Link> = col
        .find(doc! {})
        .await
        .map_err(|e| e.to_string())?
        .try_collect()
        .await
        .map_err(|e| e.to_string())?;
    Ok(data)
}
pub async fn get_link_item(db: &Database, item: String) -> Result<LinkDetail, String> {
    let col: Collection<LinkDetail> = db.collection::<LinkDetail>("links");
    let result = col
        .find_one(doc! { "_id": item})
        .await
        .map_err(|e| e.to_string())?;
    result.ok_or("Link not found".to_string())
}

fn read_file(path: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut file =
        fs::File::open(&path).map_err(|e| format!("Erreur ouverture fichier '{}': {}", path, e))?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}
