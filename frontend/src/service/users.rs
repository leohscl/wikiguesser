use super::fetch::Fetch;
use crate::entities::interfaces::InputUser;
use crate::entities::interfaces::Status;
use crate::entities::interfaces::User;
use crate::API_URL;
use crate::utils::hashing::hash_password;
use crate::utils::hashing::verify_password;

pub async fn get_user(email: &str) -> Result<User, Status> {
    let url = format!("{}/users/{}", API_URL, email);
    log::info!("url: {}", url);
    let json = Fetch::get(url).await;
    log::info!("json: {:?}", json);
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<User>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}

pub async fn create_user(user: &InputUser) -> Result<User, Status> {
    let url = format!("{}/users/", API_URL);
    log::info!("url: {}", url);
    let password_hash = match hash_password(&user.password) {
        Ok(hash) => hash,
        Err(_err) => return Err(Status::Error),
    };
    let string_user_dict = format!("{{\"email\": \"{}\", \"password\":\"{}\"}}", user.email, password_hash);
    let bool_verify = match verify_password(&user.password, &password_hash) {
        Ok(matched) => matched,
        Err(_err) => return Err(Status::Error),
    };
    log::info!("verification: {}", bool_verify);
    let jsuser = wasm_bindgen::JsValue::from_str(&string_user_dict);

    let json = Fetch::post(url, &jsuser).await;
    match json {
        Ok(json) => Ok(serde_wasm_bindgen::from_value::<User>(json).unwrap()),
        Err(_err) => Err(Status::Error),
    }
}
