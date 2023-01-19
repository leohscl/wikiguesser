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

#[derive(Debug, Deserialize)]
pub struct OngoingGame {
    pub game: Game,
    pub article: Article,
    pub all_results: Vec<Option<WordResult>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Game {
    pub id: i32,
    article_id: i32,
    ip_or_email: String,
    is_ip: bool,
    is_finished: bool,
    words: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GamePrompt {
    pub cat: String,
    pub email: String,
}

