use {
    crate::{errors::database_error::DatabaseError, models::articles::Article, Pool},
    actix_web::{web, Error, HttpResponse},
};

// /articles/{id}/
pub async fn get(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Article::get(&id, &mut connection))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(DatabaseError)?)
}
// /articles/random/pick
pub async fn get_one(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Article::get_one(&mut connection))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(DatabaseError)?)
}
