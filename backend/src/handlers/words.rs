use crate::{errors::database_error::DatabaseError, models::words::WordResult, models::words::WordModel};
use actix_web::{web, Error, HttpResponse};

// /words/{word}
pub async fn query(word: web::Path<String>, model: web::Data<WordModel>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || WordResult::query(&word, &model.embedding))
        .await
        .map(|reveal| HttpResponse::Ok().json(reveal))
        .map_err(DatabaseError)?)
}
