use crate::schema::{articles, categories};
use diesel::PgConnection;
use crate::diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};

// use common::models::{Article, WordResult};
use rand::Rng;
use serde::Serialize;

// TODO(leo): deduplicate code
#[derive(Identifiable, Debug, Serialize, Clone, Queryable)]
pub struct Article {
    pub id: i32,
    pub wiki_id: i32,
    pub title: String,
    pub content: String,
    pub views: i32,
}

impl Article {
    pub fn get(id: &i32, connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let article = articles::table.find(id).first::<Article>(connection)?;
        Ok(article)
    }
    pub fn get_one(connection: &mut PgConnection) -> Result<Article, diesel::result::Error> {
        let vec_article = articles::table.filter(articles::views.gt(100)).load::<Article>(connection)?;
        // let vec_article = articles::table.filter(articles::views.gt(50)).load::<Article>(connection)?;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..vec_article.len());
        println!("Number of articles passing filter: {}", vec_article.len());
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
        let article = vec_article.get(index).expect("There should be a first element").clone();
        Ok(article)
    }
}
