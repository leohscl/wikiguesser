use crate::schema::articles;
use serde::Serialize;
use diesel::Insertable;

#[derive(Serialize, Debug, Insertable)]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: i32,
    pub wiki_id: i32,
    pub title: String,
    pub content: String,
}
