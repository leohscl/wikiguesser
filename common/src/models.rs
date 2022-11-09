use crate::schema::articles;
use serde::Serialize;
use serde::Deserialize;
use diesel::Insertable;

#[derive(Serialize, Deserialize, Debug, Insertable)]
#[diesel(table_name = articles)]
pub struct Article {
    pub id: i32,
    pub wiki_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize, Serialize, Debug) ]
pub struct IString {
    pub str: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WordResult {
    pub word: String,
    pub close_words: Vec<IString>,
    pub variants: Vec<IString>,
}
