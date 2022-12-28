use super::fetch::Fetch;
use crate::entities::interfaces::Status;
use crate::entities::interfaces::{IString,WordResult};
// use crate::entities::interfaces::WordResult;
// use common::models::WordResult;
use crate::API_URL;

pub async fn query(word: &str) -> Result<WordResult, Status> {
    // check if word is a number
    // log::info!("word: {}", word);
    if let Ok(num) = word.parse::<i32>() {
        // log::info!("num: {}", num);
        let close_words: Vec<_> = (1..500).flat_map(|n| {
        // let close_words: Vec<_> = (1..5).flat_map(|n| {
            [num + n, num - n].into_iter()
        })
        .map(|n| IString{str:n.to_string()})
        .collect();
        let word_res = WordResult {
            word: word.to_string(),
            close_words,
            variants: vec![],
        };
        // log::info!("word_res: {:?}", word_res);
        Ok(word_res)
    } else {
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
}
