use super::articles::Article;
use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::models::words::WordModel;
use crate::models::words::WordResult;
use crate::{handlers::games::InputGame, schema::*};
use chrono::prelude::*;
use diesel::{PgConnection, QueryDsl};
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct GamePrompt {
    pub cat: String,
    pub email: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct GamePromptId {
    pub id: i32,
    pub email: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct OngoingGame {
    game: Game,
    article: Article,
    words: Vec<String>,
    // all_results: Vec<Option<WordResult>>,
}

#[derive(Identifiable, Debug, Serialize, Clone, Queryable, Insertable)]
pub struct Game {
    id: i32,
    article_id: i32,
    ip_or_email: String,
    is_ip: bool,
    is_finished: bool,
    words: String,
}

impl Game {
    fn create_with_id(
        connection: &mut PgConnection,
        game: &InputGame,
        article_id: i32,
    ) -> Result<Game, diesel::result::Error> {
        // create article
        let new_article = Article::get_one_with_id(connection, article_id)?;
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        let new_game = Game {
            id,
            article_id: new_article.id,
            ip_or_email: game.ip_or_email.to_owned(),
            is_ip: game.is_ip,
            is_finished: false,
            words: "".to_owned(),
        };
        diesel::insert_into(games::table)
            .values(&new_game)
            .execute(connection)?;
        println!("New game: {:?}", new_game);
        Ok(new_game)
    }
    fn create(
        connection: &mut PgConnection,
        game: &InputGame,
        opt_cat: &Option<String>,
    ) -> Result<Game, diesel::result::Error> {
        // create article
        let new_article = if let Some(cat) = opt_cat {
            Article::get_one_incl_filter(connection, cat)?
        } else {
            Article::get_one(connection)?
        };
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        let new_game = Game {
            id,
            article_id: new_article.id,
            ip_or_email: game.ip_or_email.to_owned(),
            is_ip: game.is_ip,
            is_finished: false,
            words: "".to_owned(),
        };
        diesel::insert_into(games::table)
            .values(&new_game)
            .execute(connection)?;
        Ok(new_game)
    }
    fn create_daily(
        connection: &mut PgConnection,
        game: &InputGame,
        server_start: &NaiveDate,
    ) -> Result<Game, diesel::result::Error> {
        // create article
        let new_article =
            Article::get_daily(connection, *server_start).expect("There should be a daily page");
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        let new_game = Game {
            id,
            article_id: new_article.id,
            ip_or_email: game.ip_or_email.to_owned(),
            is_ip: game.is_ip,
            is_finished: false,
            words: "".to_owned(),
        };
        diesel::insert_into(games::table)
            .values(&new_game)
            .execute(connection)?;
        Ok(new_game)
    }

    pub fn get_or_create_with_id(
        connection: &mut PgConnection,
        input_game: &InputGame,
        article_id: i32,
    ) -> Result<OngoingGame, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::ip_or_email.eq(input_game.ip_or_email.to_owned()));
        let query = query.filter(games::is_finished.eq(false));
        let results = query.load::<Game>(connection)?;
        let game = if let Some(game) = results.into_iter().next() {
            game
        } else {
            Self::create_with_id(connection, input_game, article_id)?
        };
        // let all_results = Self::get_all_results(&game, word_model)?;
        let words: Vec<String> = game.words.split(" ").map(|str| String::from(str)).collect();
        let article = Article::get(game.article_id, connection)?;
        Ok(OngoingGame {
            game,
            article,
            words,
        })
    }

    pub fn get_or_create_daily(
        connection: &mut PgConnection,
        input_game: &InputGame,
        server_start: &NaiveDate,
    ) -> Result<OngoingGame, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::ip_or_email.eq(input_game.ip_or_email.to_owned()));
        let query = query.filter(games::is_finished.eq(false));
        let results = query.load::<Game>(connection)?;
        let game = if let Some(game) = results.into_iter().next() {
            game
        } else {
            Self::create_daily(connection, input_game, server_start)?
        };
        let words: Vec<String> = game.words.split(" ").map(|str| String::from(str)).collect();
        let article = Article::get(game.article_id, connection)?;
        Ok(OngoingGame {
            game,
            article,
            words,
        })
    }

    pub fn get_or_create(
        connection: &mut PgConnection,
        input_game: &InputGame,
        opt_cat: &Option<String>,
    ) -> Result<OngoingGame, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::ip_or_email.eq(input_game.ip_or_email.to_owned()));
        let query = query.filter(games::is_finished.eq(false));
        let results = query.load::<Game>(connection)?;
        let game = if let Some(game) = results.into_iter().next() {
            game
        } else {
            Self::create(connection, input_game, opt_cat)?
        };
        let words: Vec<String> = game.words.split(" ").map(|str| String::from(str)).collect();
        let article = Article::get(game.article_id, connection)?;
        Ok(OngoingGame {
            game,
            article,
            words,
        })
    }
    pub fn get_ongoing(
        connection: &mut PgConnection,
        input_game: &InputGame,
    ) -> Result<Option<Game>, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::ip_or_email.eq(input_game.ip_or_email.to_owned()));
        let query = query.filter(games::is_finished.eq(false));
        let results = query.load::<Game>(connection)?;
        if let Some(game) = results.into_iter().next() {
            Ok(Some(game))
        } else {
            Ok(None)
        }
    }

    pub fn get(
        connection: &mut PgConnection,
        ip_or_email: &str,
    ) -> Result<Option<Game>, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::ip_or_email.eq(ip_or_email));
        let results = query.load::<Game>(connection)?;
        Ok(results.into_iter().next())
    }

    pub fn delete(
        connection: &mut PgConnection,
        game_id: i32,
    ) -> Result<(), diesel::result::Error> {
        diesel::delete(games::table.filter(games::id.eq(game_id))).execute(connection)?;
        Ok(())
    }

    pub fn finish(connection: &mut PgConnection, id: i32) -> Result<Game, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::id.eq(id));
        let results = query.load::<Game>(connection)?;
        println!("Finished Game: {:?}", results);
        if let Some(game) = results.into_iter().next() {
            let updated_game = diesel::update(&game)
                .set(games::is_finished.eq(true))
                .get_result::<Game>(connection)?;
            Ok(updated_game)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    pub fn update_with_id(
        connection: &mut PgConnection,
        id: i32,
        word: &str,
        word_model: &WordModel,
    ) -> Result<Option<WordResult>, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::id.eq(id));
        let results = query.load::<Game>(connection)?;
        if let Some(game) = results.into_iter().next() {
            Self::update(connection, &game, word)?;
            WordResult::query(word, &word_model.embedding)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    fn update(
        connection: &mut PgConnection,
        game: &Game,
        word: &str,
    ) -> Result<Game, diesel::result::Error> {
        let updated_words = game.words.to_owned() + " " + word;
        let updated_game = diesel::update(game)
            .set(games::words.eq(updated_words.to_owned()))
            .get_result::<Game>(connection)?;
        Ok(updated_game)
    }
}
