use super::fetch::Fetch;
use crate::entities::interfaces::JsonUser;
use crate::entities::interfaces::Status;
use crate::entities::interfaces::User;
use crate::API_URL;

pub async fn get_user(email: &str) -> Result<User, Status> {
    let url = format!("{}/users/{}", API_URL, email);
    log::info!("url: {}", url);
    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<User>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn create_user(user: JsonUser) -> Result<User, Status> {
    let url = format!("{}/users/", API_URL, email);
    log::info!("url: {}", url);
    let json = Fetch::get(url).await;
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<User>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
