use crate::models::games::Game;
use crate::models::words::WordModel;
use crate::Pool;
use crate::{
    errors::database_error::DatabaseError,
    models::games::{GamePrompt, GamePromptId},
};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use daily_functions::count_daily::count_noons_since_start;
use serde::{Deserialize, Serialize};

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
pub async fn get_ongoing(
    req: HttpRequest,
    pool: web::Data<Pool>,
    game_prompt: web::Json<GamePrompt>,
) -> Result<HttpResponse, Error> {
    println!("Request: {:?}", req);
    let val = req.connection_info();
    println!("Address {:?}", val);
    let mut connection = pool.get().unwrap();
    let opt_email = if game_prompt.email == "None" {
        None
    } else {
        Some(game_prompt.email.to_string())
    };
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &opt_email, "daily");
    let input_game = InputGame { ip_or_email, is_ip };
    Ok(
        web::block(move || Game::get_ongoing(&mut connection, &input_game))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(DatabaseError)?,
    )
}

// /games/get_or_create
pub async fn get_or_create_with_id(
    req: HttpRequest,
    pool: web::Data<Pool>,
    game_prompt: web::Json<GamePromptId>,
) -> Result<HttpResponse, Error> {
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
    let article_id = game_prompt.id;
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &opt_email, &article_id.to_string());
    let input_game = InputGame { ip_or_email, is_ip };
    Ok(
        web::block(move || Game::get_or_create_with_id(&mut connection, &input_game, article_id))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(DatabaseError)?,
    )
}
pub async fn get_or_create_daily(
    req: HttpRequest,
    pool: web::Data<Pool>,
    game_prompt: web::Json<GamePrompt>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();

    let opt_email = if game_prompt.email == "None" {
        None
    } else {
        Some(game_prompt.email.to_string())
    };
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &opt_email, "daily");
    let input_game = InputGame { ip_or_email, is_ip };
    Ok(
        web::block(move || Game::get_or_create_daily(&mut connection, &input_game))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(DatabaseError)?,
    )
}

// /games/get_or_create
pub async fn get_or_create(
    req: HttpRequest,
    pool: web::Data<Pool>,
    game_prompt: web::Json<GamePrompt>,
) -> Result<HttpResponse, Error> {
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
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &opt_email, "random");
    let input_game = InputGame { ip_or_email, is_ip };
    Ok(
        web::block(move || Game::get_or_create(&mut connection, &input_game, &opt_cat))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(DatabaseError)?,
    )
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
// /games/finished_daily
pub async fn finished_daily(
    req: HttpRequest,
    pool: web::Data<Pool>,
    game_prompt: web::Json<GamePrompt>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    let opt_email = if game_prompt.email == "None" {
        None
    } else {
        Some(game_prompt.email.to_string())
    };
    let (is_ip, ip_or_email) = get_ip_or_email(&req, &opt_email, "daily");
    let input_game = InputGame { ip_or_email, is_ip };
    Ok(
        web::block(move || Game::get_finished_daily(&mut connection, &input_game))
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
            .map_err(DatabaseError)?,
    )
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
pub async fn update(
    model: web::Data<WordModel>,
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    word: web::Json<StringPost>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(
        web::block(move || Game::update_with_id(&mut connection, *id, &word.string, &model))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(DatabaseError)?,
    )
}
fn get_ip_or_email(req: &HttpRequest, opt_email: &Option<String>, mode: &str) -> (bool, String) {
    if let Some(email) = opt_email {
        (false, email.to_string())
    } else {
        // (true, get_ip(req))
        (true, get_ip_dummy(mode))
    }
}
fn get_ip(req: &HttpRequest, mode: &str) -> String {
    // let host_value = req.headers().get(actix_web::http::header::HOST).expect("Header should contain host");
    let connection_info = req.connection_info();
    let host_value = connection_info.realip_remote_addr();
    println!("host_value: {:?}", host_value);
    // String::from(host_value.to_str().expect("Ip adress"))
    let added_id = if mode == "daily" {
        String::from(mode.to_string() + &count_noons_since_start().to_string())
    } else {
        String::from(mode)
    };
    String::from(host_value.expect("Ip adress").to_string() + &added_id)
}

#[allow(dead_code)]
fn get_ip_dummy(mode: &str) -> String {
    String::from(mode.to_string() + &count_noons_since_start().to_string())
}
