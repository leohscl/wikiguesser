use crate::models::Article;
use crate::models::Category;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenv_codegen::dotenv;
use serde::Deserialize;

use crate::op::create_article;
use crate::op::create_category;
use std::fs;

mod models;
mod op;
mod schema;

#[derive(Deserialize, Debug)]
struct JsonArticle {
    id: i32,
    title: String,
    categories: Vec<String>,
    views: i32,
    summary: String,
}
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

const DATABASE_URL: &str = dotenv!("DATABASE_URL");

#[derive(Debug, Eq, PartialEq)]
struct PageInfo {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    let file_path = "sample_cat.json";
    // println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("With text:\n{contents}");
    let data: Vec<JsonArticle> = serde_json::from_str(&contents).expect("Json should parse");
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let mut id_count = 0;
    let mut conn = pool.get().unwrap();
    for article in data.iter() {
        let future_insert = insert_one_articles(article, &mut conn, &mut id_count);
        futures::executor::block_on(future_insert);
    }
    println!("\n\nDatabase Filled !!\n\n")
}

async fn insert_one_articles(jarticle: &JsonArticle, conn: &mut PgConnection, id_count: &mut i32) {
    let article = Article {
        id: jarticle.id,
        wiki_id: jarticle.id,
        title: jarticle.title.to_owned(),
        content: jarticle.summary.to_owned(),
        views: jarticle.views,
    };
    create_article(conn, &article);
    // println!("id current: {}", *id_count);
    for category_name in jarticle.categories.clone().into_iter() {
        let category_link = Category {
            id: *id_count,
            article_id: jarticle.id,
            category: category_name,
        };
        create_category(conn, &category_link);
        *id_count += 1;
    }
}

// async fn insert_one_articles_old(jarticle: &JsonArticle, conn: &mut PgConnection) {
//     let api = mediawiki::api::Api::new("https://fr.wikipedia.org/w/api.php").await.unwrap();
//     let ids_str = jarticle.id.to_string();
//     // let params_content = api.params_into(&[
//     //     ("action", "query"),
//     //     ("format", "json"),
//     //     ("pageids", &ids_str),
//     //     ("prop", "pageviews"),
//     //     ("pvipdays", "10"),
//     // ]);
//     // // extract view count
//     // let res_content = api.get_query_api_json(&params_content).await.unwrap();
//     // let raw_page = &res_content["query"]["pages"];
//     // // println!("raw_page: {:?}", raw_page);
//     // let date = "2022-12-26";
//     // let views = &raw_page[&ids_str]["pageviews"][date]
//     //     .as_i64()
//     //     .expect("Query conversion to string failed");
//     // println!("views: {:?}", views);
//
//     let params_content = api.params_into(&[
//         ("action", "query"),
//         ("prop", "extracts"),
//         ("format", "json"),
//         ("exsentences", "10"),
//         ("exsectionformat", "wiki"),
//         ("pageids", &ids_str),
//         ("formatversion", "2"),
//         ("explaintext", "true"),
//     ]);
//     // println!("params_content: {:?}", params_content);
//     // println!("Api url: {}", api.api_url());
//     let res_content = api.get_query_api_json(&params_content).await.unwrap();
//     // println!("res_content: {:?}", res_content);
//     let raw_page = &res_content["query"]["pages"][0];
//     let title_raw_str = raw_page["title"]
//         .as_str()
//         .expect("Query conversion to string failed").to_string();
//     let content_raw_str = raw_page["extract"]
//         .as_str()
//         .expect("Query conversion to string failed").to_string();
//     let id_i32 = jarticle.id;
//     let parsed_content = parse_raw_content(content_raw_str);
//     let article = Article{id:id_i32, wiki_id:id_i32, title:title_raw_str, content:parsed_content, views: jarticle.views};
//     create_article(conn, &article);
//     let mut rng = rand::thread_rng();
//     for category_name in jarticle.categories.clone().into_iter() {
//         let id_cat: i32 = rng.gen();
//         let category_link = Category{id: id_cat, article_id: jarticle.id, category: category_name};
//         create_category(conn, &category_link);
//     }
// }
//
// fn parse_raw_content(res_content: String) -> String {
//     let splitted = res_content.split("==");
//     return splitted.into_iter().next().expect("There should be a first part").to_string();
// }
