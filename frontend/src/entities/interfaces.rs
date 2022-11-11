use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize)]
pub enum Status {
    Success,
    Error,
    Unknown,
}
