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

#[derive(Serialize, Deserialize, Debug)]
pub struct InputUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct User {
    pub id: i32,
    pub id_session: i64,
    pub t_email: String,
    pub t_password: String,
    pub t_ip_address: String,
    pub d_visit_first: NaiveDate,
}


