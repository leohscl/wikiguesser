use {
    crate::{errors::database_error::DatabaseError, models::words::WordResult},
    actix_web::{web, Error, HttpResponse},
};
// /words/{word}
pub async fn query(word: web::Path<String>) -> Result<HttpResponse, Error> {
    // let mut connection = pool.get().unwrap();
    Ok(web::block(move || WordResult::query(&word))
        .await
        .map(|reveal| HttpResponse::Ok().json(reveal))
        .map_err(DatabaseError)?)
}
