use crate::schema::*;
use diesel::PgConnection;
use crate::diesel::{QueryDsl, RunQueryDsl};
// use common::models::{Article, WordResult};
use rand::Rng;
use serde::Serialize;

// TODO(leo): deduplicate code
#[derive(Identifiable, Debug, Serialize, Clone, Queryable)]
pub struct Article {
    pub id: i32,
    // pub pub_date: String,
    pub wiki_id: i32,
    pub title: String,
    pub content: String,
}

impl Article {
    pub fn get(id: &i32, connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let article = articles::table.find(id).first::<Article>(connection)?;
        Ok(article)
    }
    pub fn get_one(connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let vec_article = articles::table.load::<Article>(connection)?;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..vec_article.len());
        let article = vec_article.get(index).expect("There should be a first element").clone();
        Ok(article)
    }
}
