use models::words::WordModel;
use finalfusion::prelude::*;
use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate dotenv_codegen;

use {
    actix_cors::Cors,
    actix_files as fs,
    actix_web::{http, web, App, HttpServer},
    diesel::r2d2::{self, ConnectionManager},
    diesel::PgConnection,
};

mod errors;
mod handlers;
mod models;
mod schema;

// Constants
const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const FILE_MODEL: &str = dotenv!("FILE_MODEL");
const NUM_WORD_RESULTS: usize = 300;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "actix_web=debug");

    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3001")
                    .allowed_origin("http://localhost:3333")
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("http://localhost:8080/signup/")
                    .allowed_origin("http://localhost:8080/login")
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://127.0.0.1:8080/signup/")
                    .allowed_methods(vec!["GET", "OPTIONS", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(
                web::Data::new({
                    let fifu_file = FILE_MODEL;
                    let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
                    let embed: Embeddings<VocabWrap, StorageViewWrap> = Embeddings::read_embeddings(&mut reader).unwrap();
                    WordModel {
                        embedding: embed,
                    }
                })
            )
            .service(fs::Files::new("/media", "./media").show_files_listing())
            .data(pool.clone())
            .route("/words/{word}", web::get().to(handlers::words::query))
            .route("/articles/{id}", web::get().to(handlers::articles::get))
            .route("/articles/random/pick", web::get().to(handlers::articles::get_one))
            .route("/users/", web::get().to(handlers::users::get_users))
            .route("/users/{email}", web::get().to(handlers::users::get_user))
            .route("/users/", web::post().to(handlers::users::create))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
