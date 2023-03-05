use crate::models::words::WordModel;
use crate::{errors::database_error::DatabaseError, models::words::WordResult};
use actix_web::{web, Error, HttpResponse};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct BoolWrapper {
    pub boolean: bool,
}

// /words/{word}
pub async fn query(
    model: web::Data<WordModel>,
    word: web::Path<String>,
) -> Result<HttpResponse, Error> {
    Ok(
        web::block(move || WordResult::query(&word, &model.embedding))
            .await
            .map(|reveal| HttpResponse::Ok().json(reveal))
            .map_err(DatabaseError)?,
    )
}
// /words/{word}
pub async fn check(
    model: web::Data<WordModel>,
    word: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let result = web::block(move || Ok(WordResult::check(&word, &model.embedding)))
        .await
        .map_err(DatabaseError)?;
    let response = HttpResponse::Ok().json(BoolWrapper { boolean: result });
    Ok(response)
}
