use diesel::PgConnection;
use diesel::prelude::*;
// use common::schema::articles::dsl::*;
use crate::schema::articles::dsl::*;
use crate::schema::categories::dsl::*;
use crate::models::Article;
use crate::models::Category;

pub fn create_article(conn: &mut PgConnection, article: &Article) {
    println!("Article: {:?}", article);
    diesel::insert_into(articles)
        .values(article)
        .execute(conn).expect("Error inserting article");
}
pub fn create_category(conn: &mut PgConnection, cat: &Category) {
    println!("Category: {:?}", category);
    diesel::insert_into(categories)
        .values(cat)
        .execute(conn).expect("Error inserting article");
}
