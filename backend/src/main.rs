use crate::models::words::WordModel;
use crate::models::words::WordResult;
use chrono::prelude::*;
use chrono_tz::Europe::Paris;
use finalfusion::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    let today = Utc::now().with_timezone(&Paris).date_naive();

    let fifu_file = FILE_MODEL;
    let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
    let embed: Embeddings<VocabWrap, StorageViewWrap> =
        Embeddings::read_embeddings(&mut reader).unwrap();
    let common_words = get_common_words();
    println!("Creating engine");
    println!("Using file model {}", FILE_MODEL);
    let result_common =
        WordResult::query_multiple(&common_words, &embed).expect("common words should not fail");
    drop(embed);
    println!("Common words model constructed");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            // .wrap(
            //     Cors::default()
            //         .allowed_origin("http://localhost:3001")
            //         .allowed_origin("http://localhost:3333")
            //         .allowed_origin("http://localhost:8080")
            //         .allowed_origin("http://localhost:8080/signup/")
            //         .allowed_origin("http://localhost:8080/login")
            //         .allowed_origin("http://127.0.0.1:8080")
            //         .allowed_origin("http://127.0.0.1:8080/signup/")
            //         .allowed_methods(vec!["GET", "OPTIONS", "POST"])
            //         .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            //         .allowed_header(http::header::CONTENT_TYPE)
            //         .max_age(3600),
            // )
            .app_data(web::Data::new(result_common.clone()))
            .app_data(web::Data::new({
                println!("Creating word model from embeddings !");
                let fifu_file = FILE_MODEL;
                let mut reader = BufReader::new(File::open(&fifu_file).unwrap());
                let embed: Embeddings<VocabWrap, StorageViewWrap> =
                    Embeddings::read_embeddings(&mut reader).unwrap();
                WordModel { embedding: embed }
            }))
            .app_data(web::Data::new(today.clone()))
            .service(fs::Files::new("/media", "./media").show_files_listing())
            .data(pool.clone())
            .route("/words/{word}", web::get().to(handlers::words::query))
            .route("/words/check/{word}", web::get().to(handlers::words::check))
            .route("/articles/{id}", web::get().to(handlers::articles::get))
            .route(
                "/articles/random/pick",
                web::get().to(handlers::articles::get_one),
            )
            .route(
                "/articles/random/daily",
                web::get().to(handlers::articles::get_daily),
            )
            .route(
                "/articles/random_in/{category}",
                web::get().to(handlers::articles::get_one_incl_filter),
            )
            .route(
                "/articles/random_out/{category}",
                web::get().to(handlers::articles::get_one_excl_filter),
            )
            .route(
                "/articles/dummy_engine/",
                web::get().to(handlers::articles::get_dummy_engine),
            )
            .route(
                "/articles/get_engine/{article_id}",
                web::get().to(handlers::articles::get_engine),
            )
            .route(
                "/articles/get_match/{word}",
                web::get().to(handlers::articles::get_match),
            )
            .route("/ratings", web::post().to(handlers::ratings::create))
            .route(
                "/reports/{article_id}",
                web::get().to(handlers::reports::get_article_reports),
            )
            .route("/reports", web::post().to(handlers::reports::create))
            .route("/reports", web::get().to(handlers::reports::get_all))
            .route("/users/", web::get().to(handlers::users::get_users))
            .route("/users/{email}", web::get().to(handlers::users::get_user))
            .route("/users/", web::post().to(handlers::users::create))
            .route(
                "/games/get_or_create_with_id",
                web::post().to(handlers::games::get_or_create_with_id),
            )
            .route(
                "/games/get_or_create_daily",
                web::post().to(handlers::games::get_or_create_daily),
            )
            .route(
                "/games/get_or_create",
                web::post().to(handlers::games::get_or_create),
            )
            .route(
                "/games/get_ongoing",
                web::post().to(handlers::games::get_ongoing),
            )
            .route(
                "/games/finished_daily",
                web::post().to(handlers::games::finished_daily),
            )
            .route(
                "/games/update/{id}",
                web::post().to(handlers::games::update),
            )
            .route(
                "/games/delete/{id}",
                web::delete().to(handlers::games::delete),
            )
            .route("/games/finish/{id}", web::get().to(handlers::games::finish))
    })
    .workers(1)
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn get_common_words() -> Vec<String> {
    let filename = "data/result_500";
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    BufReader::new(file)
        .lines()
        .filter_map(|l| l.ok())
        .collect()
}
