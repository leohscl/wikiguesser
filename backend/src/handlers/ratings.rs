use crate::errors::database_error::DatabaseError;
use crate::{models::ratings::Rating, Pool};
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRatings {
    pub article_id: i32,
    pub rating: i32,
}

// /ratings
pub async fn create(
    pool: web::Data<Pool>,
    rating: web::Json<InputRatings>,
) -> Result<HttpResponse, Error> {
    println!("Posting rating !");
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || Rating::create(&mut connection, &rating))
        .await
        .map(|rating| HttpResponse::Ok().json(rating))
        .map_err(DatabaseError)?)
}
