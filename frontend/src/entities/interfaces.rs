use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct BoolWrapper {
    pub boolean: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Article {
    pub id: i32,
    pub wiki_id: i32,
    pub title: String,
    pub content: String,
    pub views: i32,
}

impl Article {
    pub fn dummy() -> Self {
        let dummy_str = "thé";
        Article {
            id: 1,
            wiki_id: 1,
            title: dummy_str.to_string(),
            content: dummy_str.to_string(),
            views: 1000000,
        }
    }
}
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct IString {
    pub str: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct WordResult {
    pub word: String,
    pub close_words: Vec<IString>,
    pub variants: Vec<IString>,
    pub frequency: Option<f64>,
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

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct OngoingGame {
    pub game: Game,
    pub article: Article,
    // pub words: Vec<String>,
    pub all_results: Vec<Option<WordResult>>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
    pub mode: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InputReport {
    pub article_id: i32,
    pub report_cat: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InputRatings {
    pub article_id: i32,
    pub rating: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct StringAndPos {
    pub str: String,
    pub pos: usize,
}

impl PartialOrd for StringAndPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.pos.partial_cmp(&other.pos)
    }
}
impl Ord for StringAndPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct GameEngine {
    pub reveals: HashMap<String, Vec<StringAndPos>>,
    pub list_results_content: Vec<Option<WordResult>>,
    pub list_results_title: Vec<Option<WordResult>>,
}
