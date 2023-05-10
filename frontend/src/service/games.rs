use super::fetch::Fetch;
use crate::entities::interfaces::Game;
use crate::entities::interfaces::OngoingGame;
use crate::entities::interfaces::Status;
use crate::entities::interfaces::WordResult;
use crate::API_URL;

pub async fn get_ongoing_game() -> Result<Option<Game>, Status> {
    let url = format!("{}/games/get_ongoing", API_URL);
    let email = "None".to_string();
    let mode = "daily".to_string();
    let game_prompt_str = format!(
        "{{\"cat\": \"{}\", \"email\":\"{}\", \"mode\":\"{}\"}}",
        "None", email, mode
    );
    let js_game_prompt = wasm_bindgen::JsValue::from_str(&game_prompt_str);
    let res_json = Fetch::post(url, &js_game_prompt).await;
    match res_json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Option<Game>>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn get_game_with_id(id: i32) -> Result<Option<OngoingGame>, Status> {
    let url = format!("{}/games/get_or_create_with_id", API_URL);
    log::info!("url: {}", url);
    let email = "None".to_string();
    let game_prompt_str = format!(
        "{{\"id\": {}, \"email\":\"{}\", \"mode\":\"id\"}}",
        id, email
    );
    let js_game_prompt = wasm_bindgen::JsValue::from_str(&game_prompt_str);
    let res_json = Fetch::post(url, &js_game_prompt).await;
    log::info!("json: {:?}", res_json);
    // check if game was finished
    match res_json {
        Ok(json) => Ok(Some(
            serde_wasm_bindgen::from_value::<OngoingGame>(json).unwrap(),
        )),
        Err(_err) => Err(Status::Error),
    }
}
pub async fn get_game(opt_cat: Option<String>, daily: bool) -> Result<Option<OngoingGame>, Status> {
    let url = if daily {
        format!("{}/games/get_or_create_daily", API_URL)
    } else {
        format!("{}/games/get_or_create", API_URL)
    };
    let cat = if let Some(category) = opt_cat {
        category
    } else {
        "None".to_string()
    };
    let email = "None".to_string();
    let mode = if daily { "daily" } else { "random" };
    let game_prompt_str = format!(
        "{{\"cat\": \"{}\", \"email\":\"{}\", \"mode\":\"{}\"}}",
        cat, email, mode
    );
    let js_game_prompt = wasm_bindgen::JsValue::from_str(&game_prompt_str);
    let res_json = Fetch::post(url, &js_game_prompt).await;
    let finished = if daily {
        let url_finished = format!("{}/games/finished_daily", API_URL);
        let res_json = Fetch::post(url_finished, &js_game_prompt).await;
        match res_json {
            Ok(json) => serde_wasm_bindgen::from_value::<bool>(json).unwrap(),
            Err(_err) => true,
        }
    } else {
        false
    };
    if finished {
        Ok(None)
    } else {
        match res_json {
            Ok(json) => Ok(Some(
                serde_wasm_bindgen::from_value::<OngoingGame>(json).unwrap(),
            )),
            Err(_err) => Err(Status::Error),
        }
    }
}

pub async fn finish_game(id: i32) -> Result<Game, Status> {
    let url = format!("{}/games/finish/{}", API_URL, id);
    let res_json = Fetch::get(url).await;
    match res_json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Game>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn update_game(id: i32, word: &str) -> Result<Option<WordResult>, Status> {
    let url = format!("{}/games/update/{}", API_URL, id);
    let string_word = format!("{{\"string\": \"{}\"}}", word);
    let jsword = wasm_bindgen::JsValue::from_str(&string_word);
    let res_json = Fetch::post(url, &jsword).await;
    match res_json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<Option<WordResult>>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
