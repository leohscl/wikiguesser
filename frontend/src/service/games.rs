use super::fetch::Fetch;
use crate::entities::interfaces::Article;
use crate::entities::interfaces::OngoingGame;
use crate::entities::interfaces::Status;
use crate::API_URL;
use crate::entities::interfaces::WordResult;

pub async fn get_game() -> Result<OngoingGame, Status> {
    let url = format!("{}/games/get_or_create/none", API_URL);
    let res_json = Fetch::get(url).await;
    log::info!("json: {:?}", res_json);
    match res_json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<OngoingGame>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn update_game(word: &str) -> Result<Option<WordResult>, Status> {
    let url = format!("{}/games/update/none", API_URL);
    let string_word = format!("{{\"string\": \"{}\"}}", word);
    let jsword = wasm_bindgen::JsValue::from_str(&string_word);
    let res_json = Fetch::post(url, &jsword).await;
    log::info!("json: {:?}", res_json);
    match res_json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Option<WordResult>>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
