use crate::{errors::database_error::DatabaseError, models::words::WordResult};
use actix_web::{web, Error, HttpResponse};
use crate::handlers::utils::get_word_model;

// /words/{word}
pub async fn query(word: web::Path<String>) -> Result<HttpResponse, Error> {
    let model = get_word_model();
    Ok(web::block(move || WordResult::query(&word, &model.embedding))
        .await
        .map(|reveal| HttpResponse::Ok().json(reveal))
        .map_err(DatabaseError)?)
}
