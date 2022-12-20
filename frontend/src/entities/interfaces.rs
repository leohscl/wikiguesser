use serde::Serialize;
use serde::Deserialize;
use chrono::NaiveDate;

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

pub struct JsonUser {
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct User {
    id: i32,
    id_session: i64,
    t_email: String,
    t_password: String,
    t_ip_address: String,
    d_visit_first: NaiveDate,
}


