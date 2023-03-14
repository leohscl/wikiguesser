use super::fetch::Fetch;
use crate::entities::interfaces::InputRatings;
use crate::entities::interfaces::Status;
use crate::API_URL;

pub async fn create_rating(rating: &InputRatings) -> Result<(), Status> {
    let url = format!("{}/ratings", API_URL);
    let string_rating = format!(
        "{{\"article_id\": {}, \"rating\":{}}}",
        rating.article_id, rating.rating
    );
    let jsrating = wasm_bindgen::JsValue::from_str(&string_rating);
    let json = Fetch::post(url, &jsrating).await;
    match json {
        Ok(_json) => Ok(()),
        Err(_err) => Err(Status::Error),
    }
}
