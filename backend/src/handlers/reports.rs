use actix_web::{web, HttpResponse, Error};
use serde::{Serialize, Deserialize};
use crate::{Pool, models::reports::Reports};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputReport {
    pub article_id: i32,
    pub report_cat: String,
    pub description: String,
}

// /reports/
pub async fn create(pool: web::Data<Pool>, user: web::Json<InputReport>) -> Result<HttpResponse, Error> {
    println!("Posting report !");
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Reports::create(&mut connection, user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
