use super::fetch::Fetch;
use crate::entities::interfaces::Article;
// use common::models::Article;
use crate::entities::interfaces::Status;
use crate::API_URL;


pub async fn get_one_article(opt_cat: Option<String>) -> Result<Article, Status> {
    let url = if let Some(cat) = opt_cat {
        format!("{}/articles/random_in/{}", API_URL, cat)
    } else {
        format!("{}/articles/random/pick", API_URL)
    };
    // log::info!("url: {}", url);
    let json = Fetch::get(url).await;
    log::info!("json: {:?}", json);
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Article>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_article(id: &i32) -> Result<Article, Status> {
    let url = format!("{}/articles/{}", API_URL, id);
    // log::info!("url: {}", url);
    let json = Fetch::get(url).await;
    log::info!("json: {:?}", json);
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Article>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

