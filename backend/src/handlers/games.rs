use actix_web::{web, HttpResponse, HttpRequest, Error};
use crate::{errors::database_error::DatabaseError, models::games::GamePrompt};
use crate::Pool;
use crate::models::games::Game;
use serde::{Serialize, Deserialize};
use crate::models::words::WordModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputGame {
    pub ip_or_email: String,
    pub is_ip: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringPost {
    string: String,
}

// /games/get_ongoing
pub async fn get_ongoing(req: HttpRequest, pool: web::Data<Pool>, game_prompt: web::Json<GamePrompt>) -> Result<HttpResponse, Error> {
    println!("Request: {:?}", req);
    let val = req.connection_info();
    println!("Address {:?}", val);
    let mut connection = pool.get().unwrap();
    let opt_email = if game_prompt.email == "None" {
        None
    } else {
        Some(game_prompt.email.to_string())
    };
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &opt_email);
    let input_game = InputGame{ip_or_email, is_ip};
    Ok(web::block(move || Game::get_ongoing(&mut connection, &input_game))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}

// /games/get_or_create/{email}
pub async fn get_or_create(req: HttpRequest, pool: web::Data<Pool>, game_prompt: web::Json<GamePrompt>) -> Result<HttpResponse, Error> {
    // let model = get_word_model();
    println!("Request: {:?}", req);
    let val = req.connection_info();
    println!("Address {:?}", val);
    let mut connection = pool.get().unwrap();
    let opt_email = if game_prompt.email == "None" {
        None
    } else {
        Some(game_prompt.email.to_string())
    };
    let opt_cat = if game_prompt.cat == "None" {
        None
    } else {
        Some(game_prompt.cat.to_string())
    };
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &opt_email);
    let input_game = InputGame{ip_or_email, is_ip};
    Ok(web::block(move || Game::get_or_create(&mut connection, &input_game, &opt_cat))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}

// /games/finish/{id}
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    println!("deleting game id {}", id);
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Game::delete(&mut connection, *id))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
// /games/finish/{id}
pub async fn finish(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Game::finish(&mut connection, *id))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
// /games/update/{id}
pub async fn update(model: web::Data<WordModel>, pool: web::Data<Pool>, id: web::Path<i32>, word: web::Json<StringPost>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Game::update_with_id(&mut connection, *id, &word.string, &model))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
fn get_ip_or_email(req: &HttpRequest, opt_email: &Option<String>) -> (bool, String) {
    if let Some(email) = opt_email {
        (false, email.to_string())
    } else {
        (true, get_ip(req))
    }
}
fn get_ip(req: &HttpRequest) -> String {
    // let host_value = req.headers().get(actix_web::http::header::HOST).expect("Header should contain host");
    let connection_info = req.connection_info();
    let host_value = connection_info.realip_remote_addr();
    println!("host_value: {:?}", host_value);
    // String::from(host_value.to_str().expect("Ip adress"))
    String::from(host_value.expect("Ip adress"))
}
