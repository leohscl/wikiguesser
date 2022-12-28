use diesel::PgConnection;
use diesel::prelude::*;
// use common::schema::articles::dsl::*;
use crate::schema::articles::dsl::*;
use crate::models::Article;

pub fn create_article(conn: &mut PgConnection, article: Article) {
    println!("Article: {:?}", article);
    diesel::insert_into(articles)
        .values(&article)
        .execute(conn).expect("Error inserting article");
}
