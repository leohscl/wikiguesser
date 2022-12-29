use crate::schema::articles;
use crate::schema::categories;
use serde::Serialize;
use diesel::Insertable;

#[derive(Serialize, Debug, Insertable)]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: i32,
    pub wiki_id: i32,
    pub title: String,
    pub content: String,
    pub views: i32,
}

#[derive(Serialize, Debug, Insertable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub article_id: i32,
    pub category: String,
}
