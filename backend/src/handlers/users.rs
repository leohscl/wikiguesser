use crate::errors::database_error::DatabaseError;
use crate::models::users::User;
use crate::Pool;
use actix_web::{web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub email: String,
    pub password: String,
}

// /users
pub async fn get_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || User::get_all(&mut connection))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
// /users/{email}
pub async fn get_user(
    email: web::Path<String>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().unwrap();
    Ok(
        web::block(move || User::get_by_email(&mut connection, &email))
            .await
            .map(|user| HttpResponse::Ok().json(user))
            .map_err(DatabaseError)?,
    )
}
// /users/
pub async fn create(
    pool: web::Data<Pool>,
    user: web::Json<InputUser>,
) -> Result<HttpResponse, Error> {
    println!("Posting !");
    let mut connection = pool.get().unwrap();
    Ok(web::block(move || User::create(&mut connection, user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(DatabaseError)?)
}
