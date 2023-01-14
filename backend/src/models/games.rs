use crate::{schema::*, handlers::games::InputGame};
use diesel::{PgConnection, QueryDsl};
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use serde::Serialize;
use rand::Rng;

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
    fn create(connection: &mut PgConnection, game: &InputGame) -> Result<Game, diesel::result::Error> {
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        let new_game = Game {
            id,
            article_id: game.article_id,
            ip_or_email: game.ip_or_email.to_owned(),
            is_ip: game.is_ip,
            is_finished: true,
            words: "".to_owned(),
        };
        diesel::insert_into(games::table)
            .values(&new_game)
            .execute(connection)?;
        Ok(new_game)
    }
    pub fn get_or_create(connection: &mut PgConnection, input_game: &InputGame) -> Result<Game, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::ip_or_email.eq(input_game.ip_or_email.to_owned()));
        let results = query.load::<Game>(connection)?;
        println!("Game: {:?}", results);
        if let Some(game) = results.into_iter().next() {
            Ok(game)
        } else {
            Self::create(connection, input_game)
        }
    }

    pub fn get(connection: &mut PgConnection, ip_or_email: &str) -> Result<Option<Game>, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::ip_or_email.eq(ip_or_email));
        let results = query.load::<Game>(connection)?;
        println!("Game: {:?}", results);
        Ok(results.into_iter().next())
    }

    pub fn update_with_id(connection: &mut PgConnection, game_id: i32, word: &str) -> Result<Game, diesel::result::Error> {
        let query = games::table.into_boxed();
        let query = query.filter(games::id.eq(game_id));
        let results = query.load::<Game>(connection)?;
        println!("Game: {:?}", results);
        if let Some(game) = results.into_iter().next() {
            Self::update(connection, &game, word)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    fn update(connection: &mut PgConnection, game: &Game, word: &str) -> Result<Game, diesel::result::Error> {
        let updated_words = game.words.to_owned() + " " + word;
        let updated_game = diesel::update(game)
            .set(games::words.eq(updated_words.to_owned()))
            .get_result::<Game>(connection)?;
        Ok(updated_game)
    }
}
