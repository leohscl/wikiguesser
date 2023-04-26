use super::fetch::Fetch;
use crate::entities::interfaces::Article;
use crate::entities::interfaces::GameEngine;
// use common::models::Article;
use crate::entities::interfaces::Status;
use crate::API_URL;

pub async fn get_one_article(opt_cat: Option<String>) -> Result<Article, Status> {
    let url = if let Some(cat) = opt_cat {
        format!("{}/articles/random_in/{}", API_URL, cat)
    } else {
        format!("{}/articles/random/pick", API_URL)
    };
    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Article>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_matches(word: &str) -> Result<Vec<Article>, Status> {
    let mut chars_word = word.chars();
    let capitalized_word = match chars_word.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(chars_word).collect(),
    };
    let url = format!("{}/articles/get_match/{}", API_URL, capitalized_word);
    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Vec<Article>>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_article(id: &i32) -> Result<Article, Status> {
    let url = format!("{}/articles/{}", API_URL, id);
    // log::info!("url: {}", url);
    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Article>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_engine(article_id: i32) -> Result<GameEngine, Status> {
    let url = format!("{}/articles/get_engine/{}", API_URL, article_id);
    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<GameEngine>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
