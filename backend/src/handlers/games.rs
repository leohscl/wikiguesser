use actix_web::{web, HttpResponse, HttpRequest, Error};
use crate::errors::database_error::DatabaseError;
use crate::Pool;
use crate::models::games::Game;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputGame {
    pub article_id: i32,
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
pub async fn get_or_create(req: HttpRequest, pool: web::Data<Pool>, article_id: web::Path<i32>, email: web::Path<String>) -> Result<HttpResponse, Error> {
    println!("Request: {:?}", req);
    let is_ip = &email.to_string() == "none";
    let ip_or_email = if is_ip {
        let host_value = req.headers().get(actix_web::http::header::HOST).expect("Header should contain host");
        println!("host_value: {:?}", host_value);
        String::from(host_value.to_str().expect("Ip adress"))
    } else {
        email.to_owned()
    };
    // let ip_or_email = String::from("test");
    let mut connection = pool.get().unwrap();
    let input_game = InputGame{article_id: *article_id, ip_or_email, is_ip};
    Ok(web::block(move || Game::get_or_create(&mut connection, &input_game))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}

// /games/update/{word}
pub async fn update(pool: web::Data<Pool>, id: web::Path<i32>, word: web::Json<StringPost>) -> Result<HttpResponse, Error> {
    println!("updating game id {}", id);
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Game::update_with_id(&mut connection, *id, &word.string))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
