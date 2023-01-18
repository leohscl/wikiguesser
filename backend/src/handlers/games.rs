use actix_web::{web, HttpResponse, HttpRequest, Error};
use crate::errors::database_error::DatabaseError;
use crate::Pool;
use crate::models::games::Game;
use crate::models::words::WordModel;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputGame {
    pub ip_or_email: String,
    pub is_ip: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringPost {
    string: String,
}

// pub async fn create(pool: web::Data<Pool>, input_game: web::Json<InputGame>) -> Result<HttpResponse, Error> {
//     println!("Posting game !");
//     println!("input_game: {:?}", input_game);
//     let mut connection = pool.get().unwrap();
//     Ok(web::block(move || Game::create(&mut connection, input_game))
//         .await
//         .map(|user| HttpResponse::Ok().json(user))
//         .map_err(DatabaseError)?)
// }
// /games/get/{ip_or_email}
// pub async fn get(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {

// /games/get_or_create/{email}
pub async fn get_or_create(req: HttpRequest, pool: web::Data<Pool>, email: web::Path<String>, model: web::Data<WordModel>) -> Result<HttpResponse, Error> {
    println!("Request: {:?}", req);
    let val = req.connection_info();
    println!("Address {:?}", val);
    let mut connection = pool.get().unwrap();
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &email);
    let input_game = InputGame{ip_or_email, is_ip};
    Ok(web::block(move || Game::get_or_create(&mut connection, &input_game, &model))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}

// /games/{id}
pub async fn delete(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    println!("deleting game id {}", id);
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Game::delete(&mut connection, *id))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
// /games/update/{email}
pub async fn update(req: HttpRequest, pool: web::Data<Pool>, email: web::Path<String>, word: web::Json<StringPost>, model: web::Data<WordModel>) -> Result<HttpResponse, Error> {
    let (_, ip_or_email) = get_ip_or_email(&req, &email);
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Game::update_with_id(&mut connection, &ip_or_email, &word.string, &model))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
fn get_ip_or_email(req: &HttpRequest, email: &str) -> (bool, String) {
    let is_ip = &email.to_string() == "none";
    let ip_or_email = if is_ip {
        get_ip(&req)
    } else {
        email.to_owned()
    };
    (is_ip, ip_or_email)
}
fn get_ip(req: &HttpRequest) -> String {
    let host_value = req.headers().get(actix_web::http::header::HOST).expect("Header should contain host");
    println!("host_value: {:?}", host_value);
    String::from(host_value.to_str().expect("Ip adress"))
    // if let Some(val) = req.peer_addr() {
    //     println!("Address {:?}", val.ip());
    // };
}
