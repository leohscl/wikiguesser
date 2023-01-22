use actix_web::{web, HttpResponse, Error};
use serde::{Serialize, Deserialize};
use crate::{Pool, models::reports::Report};
use crate::errors::database_error::DatabaseError;

#[derive(Debug, Serialize, Deserialize)]
pub struct InputReport {
    pub article_id: i32,
    pub report_cat: String,
    pub description: String,
}

// /reports/
pub async fn create(pool: web::Data<Pool>, report: web::Json<InputReport>) -> Result<HttpResponse, Error> {
    println!("Posting report !");
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Report::create(&mut connection, &report))
        .await
        .map(|report| HttpResponse::Ok().json(report))
        .map_err(DatabaseError)?)
}

// /reports/{article_id}
pub async fn get_article_reports(pool: web::Data<Pool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Report::get_article_reports(&mut connection, *id))
        .await
        .map(|report| HttpResponse::Ok().json(report))
        .map_err(DatabaseError)?)
}

// /reports/
pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Report::get_all(&mut connection))
        .await
        .map(|report| HttpResponse::Ok().json(report))
        .map_err(DatabaseError)?)
}
