use super::fetch::Fetch;
use crate::entities::interfaces::Status;
use common::models::WordResult;
use crate::API_URL;

pub async fn query(word: &str) -> Result<WordResult, Status> {
    let url = format!("{}/words/{}", API_URL, word);
    // log::info!("url: {}", url);
    let json = Fetch::get(url).await;
    // log::info!("json: {:?}", json);
    match json {
        // Ok(json) => Ok(json.into_serde::<WordResult>().unwrap()),
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<WordResult>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
