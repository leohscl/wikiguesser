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
                    .allowed_methods(vec!["GET", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .service(fs::Files::new("/media", "./media").show_files_listing())
            .data(pool.clone())
            .route("/words/{word}", web::get().to(handlers::words::query))
            .route("/articles/{id}", web::get().to(handlers::articles::get))
            .route("/articles/random/pick", web::get().to(handlers::articles::get_one))
            // .route("/articles", web::get().to(handlers::articles::list))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

