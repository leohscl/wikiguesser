use super::words::WordResult;
use crate::diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::models::words::WordModel;
use crate::{
    get_common_words,
    schema::{articles, categories},
};
use daily_functions::count_daily::count_noons_since_start;
use diesel::dsl::sql;
use diesel::PgConnection;
use finalfusion::prelude::*;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use wiki_process::wiki_parse::create_string_vector;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GameEngine {
    reveals: HashMap<String, Vec<StringAndPos>>,
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

    pub fn dummy_2() -> Self {
        let dummy_title = "thé".to_string();
        let dummy_content = "le thé c'est mieux que le café".to_string();
        Article {
            id: 1,
            wiki_id: 1,
            title: dummy_title,
            content: dummy_content,
            views: 1000000,
        }
    }

    pub fn get_dummy_engine(word_model: &WordModel) -> Result<GameEngine, diesel::result::Error> {
        let dummy_article = Article::dummy_2();
        let content = String::from(dummy_article.content.clone() + " ");
        let content_vec = create_string_vector(&content);
        Self::create_engine_with_common(&content_vec, &word_model.embedding, &Vec::new())
    }

    pub fn get_engine_from_id(
        connection: &mut PgConnection,
        article_id: i32,
        word_model: &WordModel,
        result_common: &Vec<Option<WordResult>>,
    ) -> Result<GameEngine, diesel::result::Error> {
        let query = articles::table.into_boxed();
        let query = query.filter(articles::id.eq(article_id));
        let results = query.load::<Article>(connection)?;
        if let Some(article) = results.into_iter().next() {
            println!("Article: {:?}", article);
            article.get_engine(&word_model.embedding, result_common)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    pub fn get(id: i32, connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let article = articles::table.find(id).first::<Article>(connection)?;
        Ok(article)
    }
    pub fn get_one(connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let vec_article = articles::table
            .filter(articles::views.gt(10000))
            .load::<Article>(connection)?;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..vec_article.len());
        let article = vec_article
            .get(index)
            .expect("There should be an element")
            .clone();
        Ok(article)
    }

    pub fn get_daily(connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let num_noons: usize = count_noons_since_start();
        let mut vec_article = articles::table
            .filter(articles::views.gt(10000))
            .load::<Article>(connection)?;
        let mut r = rand::rngs::StdRng::seed_from_u64(0);
        vec_article.shuffle(&mut r);
        let article = vec_article
            .get(num_noons)
            .expect("There should be an element")
            .clone();
        Ok(article)
    }

    pub fn get_one_with_id(
        connection: &mut PgConnection,
        id: i32,
    ) -> Result<Article, diesel::result::Error> {
        let join = articles::table.inner_join(categories::table);
        let id_predicate = categories::article_id.eq(id);
        let filtered = join.filter(id_predicate);
        let sel = filtered.select((
            articles::id,
            articles::wiki_id,
            articles::title,
            articles::content,
            articles::views,
        ));
        let vec_article = sel.distinct().load::<Article>(connection)?;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..vec_article.len());
        println!("Nombre d'articles: {}", vec_article.len());
        let article = vec_article
            .get(index)
            .expect("There should be an element")
            .clone();
        Ok(article)
    }

    pub fn get_one_excl_filter(
        connection: &mut PgConnection,
        cat_filter: &str,
    ) -> Result<Article, diesel::result::Error> {
        Self::get_one_right_filter(connection, cat_filter, false)
    }

    pub fn get_one_incl_filter(
        connection: &mut PgConnection,
        cat_filter: &str,
    ) -> Result<Article, diesel::result::Error> {
        Self::get_one_right_filter(connection, cat_filter, true)
    }

    fn get_one_right_filter(
        connection: &mut PgConnection,
        cat_filter: &str,
        incl: bool,
    ) -> Result<Article, diesel::result::Error> {
        let join = articles::table.inner_join(categories::table);
        // let views_predicate = articles::views.gt(100);
        let vec_article = if incl {
            let cat_predicate = categories::category.eq(cat_filter);
            // let filtered = join.filter(views_predicate).filter(cat_predicate);
            let filtered = join.filter(cat_predicate);
            let sel = filtered.select((
                articles::id,
                articles::wiki_id,
                articles::title,
                articles::content,
                articles::views,
            ));
            sel.distinct().load::<Article>(connection)?
        } else {
            let cat_predicate = categories::category.eq(cat_filter);
            // let filtered = join.filter(views_predicate).filter(cat_predicate);
            let filtered = join.filter(cat_predicate);
            let sel = filtered.select((
                articles::id,
                articles::wiki_id,
                articles::title,
                articles::content,
                articles::views,
            ));
            sel.distinct().load::<Article>(connection)?
        };
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..vec_article.len());
        println!("Nombre d'articles: {}", vec_article.len());
        let article = vec_article
            .get(index)
            .expect("There should be an element")
            .clone();
        Ok(article)
    }

    pub fn get_match(
        connection: &mut PgConnection,
        prefix: &str,
    ) -> Result<Vec<Article>, diesel::result::Error> {
        use crate::schema::articles::dsl::*;
        let articles_with_prefix = articles
            .filter(sql::<diesel::sql_types::Bool>(&format!(
                "articles.title LIKE '{prefix}%'"
            )))
            .load::<Article>(connection)?;
        Ok(articles_with_prefix)
    }

    pub fn get_engine(
        &self,
        embed: &Embeddings<VocabWrap, StorageViewWrap>,
        result_common: &Vec<Option<WordResult>>,
    ) -> Result<GameEngine, diesel::result::Error> {
        let content = String::from(self.content.clone() + " ");
        let mut content_vec = create_string_vector(&content);
        println!("Number of word in page: {}", content_vec.len());
        content_vec.sort();
        content_vec.dedup();
        println!("Number of word after dedup: {}", content_vec.len());
        // remove common words
        let common_words = get_common_words();
        for common_w in common_words.iter() {
            content_vec.retain(|word| word != common_w);
        }

        println!("Number of word to query: {}", content_vec.len());
        Self::create_engine_with_common(&content_vec, embed, result_common)
    }

    pub fn create_engine_with_common(
        words: &Vec<String>,
        embed: &Embeddings<VocabWrap, StorageViewWrap>,
        result_common: &Vec<Option<WordResult>>,
    ) -> Result<GameEngine, diesel::result::Error> {
        let mut hash = HashMap::new();

        let query_results = WordResult::query_multiple(words, embed)?;
        let iterator_common = result_common.into_iter().filter_map(|r| r.as_ref());
        for query_result in query_results
            .iter()
            .filter_map(|r| r.as_ref())
            .chain(iterator_common)
        {
            for word in &query_result.variants {
                let string_and_pos = StringAndPos {
                    str: query_result.word.to_string(),
                    pos: 0,
                };
                hash.entry(word.str.clone())
                    .or_insert(Vec::with_capacity(20))
                    .push(string_and_pos);
            }
            for (pos, word) in query_result.close_words.iter().enumerate() {
                let string_and_pos = StringAndPos {
                    str: query_result.word.to_string(),
                    pos: pos + 1,
                };
                hash.entry(word.str.clone())
                    .or_insert(Vec::with_capacity(20))
                    .push(string_and_pos);
            }
        }
        println!("Done creating engine !");
        Ok(GameEngine { reveals: hash })
    }
}
