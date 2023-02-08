use super::fetch::Fetch;
use crate::entities::interfaces::Status;
use crate::entities::interfaces::{IString,WordResult};
// use crate::entities::interfaces::WordResult;
// use common::models::WordResult;
use crate::API_URL;

pub async fn query(word: &str) -> Result<Option<WordResult>, Status> {
    if let Ok(num) = word.parse::<i32>() {
        let close_words: Vec<_> = (1..500).flat_map(|n| {
            [num + n, num - n].into_iter()
        })
        .map(|n| IString{str:n.to_string()})
        .collect();
        let word_res = WordResult {
            word: word.to_string(),
            close_words,
            variants: vec![],
        };
        Ok(Some(word_res))
    } else {
        let url = format!("{}/words/{}", API_URL, word);
        let json = Fetch::get(url).await;
        match json {
            Ok(json) => Ok(serde_wasm_bindgen::from_value::<Option<WordResult>>(json).unwrap()),
            Err(_err) => Err(Status::Error),
        }
    }
}
