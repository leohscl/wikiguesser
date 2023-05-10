use crate::models::words::{WordModel, WordResult};
use {
    crate::{errors::database_error::DatabaseError, models::articles::Article, Pool},
    actix_web::{web, Error, HttpResponse},
};

// /articles/{id}/
pub async fn get(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Article::get(*id, &mut connection))
        .await
        .map(|article| HttpResponse::Ok().json(article))
        .map_err(DatabaseError)?)
}

// /articles/get_match/{word_title}
pub async fn get_match(
    pool: web::Data<Pool>,
    word: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(
        web::block(move || Article::get_match(&mut connection, &word))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}

// /articles/random/pick
pub async fn get_daily(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Article::get_daily(&mut connection))
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
// /articles/random_in/{category}
pub async fn get_one_incl_filter(
    pool: web::Data<Pool>,
    cat: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(
        web::block(move || Article::get_one_incl_filter(&mut connection, &cat))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}

// /articles/random_out/{category}
pub async fn get_one_excl_filter(
    pool: web::Data<Pool>,
    cat: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(
        web::block(move || Article::get_one_incl_filter(&mut connection, &cat))
            .await
            .map(|article| HttpResponse::Ok().json(article))
            .map_err(DatabaseError)?,
    )
}
// /articles/get_engine/{id}
pub async fn get_engine(
    pool: web::Data<Pool>,
    article_id: web::Path<i32>,
    model: web::Data<WordModel>,
    result_common: web::Data<Vec<Option<WordResult>>>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || {
        Article::get_engine_from_id(&mut connection, *article_id, &model, &result_common)
    })
    .await
    .map(|user| HttpResponse::Ok().json(user))
    .map_err(DatabaseError)?)
}
// /articles/dummy_engine
pub async fn get_dummy_engine(model: web::Data<WordModel>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || Article::get_dummy_engine(&model))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
