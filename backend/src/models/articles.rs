use std::collections::HashMap;
use crate::schema::{articles, categories};
use diesel::PgConnection;
use crate::diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
use serde::Deserialize;
use finalfusion::prelude::*;
use crate::models::words::WordModel;

// use common::models::{Article, WordResult};
use rand::Rng;
use serde::Serialize;
use super::words::WordResult;

// TODO(leo): deduplicate code
#[derive(Identifiable, Debug, Serialize, Clone, Queryable)]
pub struct Article {
    pub id: i32,
    pub wiki_id: i32,
    pub title: String,
    pub content: String,
    pub views: i32,
}

joinable!(categories -> articles (article_id));

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
struct StringAndPos {
    str: String,
    pos: usize,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameEngine {
    reveals: HashMap<String, Vec<StringAndPos>>,
}

impl Article {

    pub fn dummy() -> Self {
        let dummy_str = "thé";
        Article{id: 1, wiki_id: 1, title: dummy_str.to_string(), content: dummy_str.to_string(), views: 1000000}
    }

    pub fn dummy_2() -> Self {
        let dummy_title = "thé".to_string();
        let dummy_content = "le thé c'est mieux que le café".to_string();
        Article{id: 1, wiki_id: 1, title: dummy_title, content: dummy_content, views: 1000000}
    }

    pub fn get_dummy_engine(word_model: &WordModel) -> Result<GameEngine, diesel::result::Error> {
        let dummy_article = Article::dummy_2();
        dummy_article.get_engine(&word_model.embedding)
    }

    pub fn get_engine_from_id(connection: &mut PgConnection, article_id: i32, word_model: &WordModel) -> Result<GameEngine, diesel::result::Error> {
        let query = articles::table.into_boxed();
        let query = query.filter(articles::id.eq(article_id));
        let results = query.load::<Article>(connection)?;
        println!("Game: {:?}", results);
        if let Some(article) = results.into_iter().next() {
            article.get_engine(&word_model.embedding)
        } else {
            Err(diesel::result::Error::NotFound)
        }

    }

    pub fn get(id: i32, connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let article = articles::table.find(id).first::<Article>(connection)?;
        Ok(article)
    }
    pub fn get_one(connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let vec_article = articles::table.filter(articles::views.gt(10000)).load::<Article>(connection)?;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..vec_article.len());
        let article = vec_article.get(index).expect("There should be a first element").clone();
        Ok(article)
    }
    pub fn get_one_excl_filter(connection: &mut PgConnection, cat_filter: &str) -> Result<Article, diesel::result::Error> {
        Self::get_one_right_filter(connection, cat_filter, false)
    }
    pub fn get_one_incl_filter(connection: &mut PgConnection, cat_filter: &str) -> Result<Article, diesel::result::Error> {
        Self::get_one_right_filter(connection, cat_filter, true)
    }
    fn get_one_right_filter(connection: &mut PgConnection, cat_filter: &str, incl: bool) -> Result<Article, diesel::result::Error> {
        let join = articles::table.inner_join(categories::table);
        let views_predicate = articles::views.gt(100);
        let vec_article = if incl {
            let cat_predicate = categories::category.eq(cat_filter);
            let filtered = join.filter(views_predicate).filter(cat_predicate);
            let sel = filtered.select((articles::id, articles::wiki_id, articles::title, articles::content, articles::views));
            sel.distinct().load::<Article>(connection)?
        } else {
            let cat_predicate = categories::category.eq(cat_filter);
            let filtered = join.filter(views_predicate).filter(cat_predicate);
            let sel = filtered.select((articles::id, articles::wiki_id, articles::title, articles::content, articles::views));
            sel.distinct().load::<Article>(connection)?
        };
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..vec_article.len());
        println!("Nombre d'articles: {}", vec_article.len());
        let article = vec_article.get(index).expect("There should be a first element").clone();
        Ok(article)
    }

    pub fn get_engine(&self, embed: &Embeddings<VocabWrap, StorageViewWrap>) -> Result<GameEngine, diesel::result::Error> {
        let content = String::from(self.content.clone() + " ");
        let mut content_vec = Self::create_string_vector(&content);
        println!("Number of word in page: {}", content_vec.len());
        content_vec.sort();
        content_vec.dedup();
        println!("Number of word to query: {}", content_vec.len());
        // can optimise this further by caching first 100 words in wiki
        Self::create_engine(&content_vec, embed)
    }

    fn create_engine(words: &Vec<String>, embed: &Embeddings<VocabWrap, StorageViewWrap>) -> Result<GameEngine, diesel::result::Error> {
        let mut hash = HashMap::new();
        let query_results = WordResult::query_multiple(words, embed)?;
        for query_result in query_results.into_iter().filter_map(|r| r) {
            for _word in query_result.variants {
                // skip for now
            }
            for (pos, word) in query_result.close_words.into_iter().enumerate() {
                let string_and_pos = StringAndPos{str: query_result.word.to_string(), pos};
                hash.entry(word.str).or_insert(Vec::with_capacity(20)).push(string_and_pos);
            }
        }
        Ok(GameEngine { reveals: hash })
    }

    fn create_string_vector(text: &str) -> Vec<String> {
        let processed_text = text.replace("\n\n\n", "").to_string();
        let processed_text = processed_text.replace("()", "").to_string();
        let separators = [' ', '\'', '.', '(', ')', ',', '!', '?', ';', ':', '/', '§', '%', '*', '€', ']', '[', '-', '\n'];
        let separator_indexes: Vec<_> = [0].into_iter().chain(
            processed_text
            .char_indices()
            .filter_map(|(index, char)| {
                match separators.iter().find(|c| *c == &char) {
                    Some(_) => {
                        Some([index, index+1])
                    },
                    None => None,
                }
            })
            .flatten())
            .collect();
        separator_indexes
            .windows(2)
            .map(|slice| {
                let start = *slice.get(0).expect("slice should have 2 elements");
                let end = *slice.get(1).expect("slice should have 2 elements");
                let chunk = &text[start..end];
                let chunk_string = chunk.to_string();
                chunk_string
            })
            .map(|str| str.to_string())
            .collect()
    }
}
