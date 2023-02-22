use crate::{errors::database_error::DatabaseError, models::words::WordResult};
use crate::models::words::WordModel;
use actix_web::{web, Error, HttpResponse};

// /words/{word}
pub async fn query(model: web::Data<WordModel>, word: web::Path<String>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || WordResult::query(&word, &model.embedding))
        .await
        .map(|reveal| HttpResponse::Ok().json(reveal))
        .map_err(DatabaseError)?)
}
