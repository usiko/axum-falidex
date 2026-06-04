use mongodb::Database;

use crate::{
    cache::FileCache,
    db::falidex::symbole::{self, set_picture_migrated},
    model::falidex_model::Img,
    resources::cloudinary::{self, migrate_picture_for_symbole},
    state::AppState,
};

pub async fn migrate_symboles_imgs(state: &AppState) {
    let db = &state.db.db;
    let symboles = symbole::get(db).await.unwrap_or(vec![]);
    let cache = FileCache::new(".app_temp/urls");
    for symbole in symboles {
        for img in symbole.imgs.unwrap_or(vec![]) {
            let res = migrate_symbole_img(db, img, &symbole.id).await;
            match res {
                Ok(mes) => {
                    println!("migrations ok {}", mes)
                }
                Err(err) => {
                    eprintln!("migrations nok {}", err)
                }
            }
        }
        let _ = cache.delete(&symbole.id).await;
    }
}

async fn migrate_symbole_img(db: &Database, img: Img, symbole_id: &str) -> Result<String, String> {
    if img.migrated == Some(true) {
        Ok(format!("{} Already migrated", img.url))
    } else {
        let mut fixed_url_img = img.url.clone();
        if (fixed_url_img.starts_with("/")) {
            fixed_url_img.remove(0);
        }
        let remote_url = format!(
            "https://resources.falidex.fr/index.php?getPicture={}&littleSalty=iBhodGT6GTqS6Bb",
            fixed_url_img
        );

        let result_upload = migrate_picture_for_symbole(&symbole_id, &remote_url).await;
        match result_upload {
            Ok(response) => {
                let result_update = set_picture_migrated(db, img, &symbole_id).await;
                match result_update {
                    Ok(response) => Ok(format!("finaly migrate pic {:?}", response)),
                    Err(error) => Err(format!("Error updating picture {}", error)),
                }
            }
            Err(error) => Err(format!("Error uploading picture {}", error)),
        }
    }
}
