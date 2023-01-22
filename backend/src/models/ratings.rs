use serde::Serialize;
use crate::handlers::ratings::InputRatings;
use crate::schema::ratings;
use diesel::PgConnection;
use crate::diesel::RunQueryDsl;
use rand::Rng;

#[derive(Debug, Serialize, Clone, Queryable, Insertable)]
#[diesel(table_name = ratings)]
pub struct Rating {
    pub id: i32,
    pub article_id: i32,
    pub date: chrono::NaiveDateTime,
    pub rating: i32,
}
impl Rating {
    pub fn create(connection: &mut PgConnection, input_rating: &InputRatings) -> Result<Rating, diesel::result::Error>{
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        let naive_date_time = chrono::Utc::now().naive_utc();
        let rating = Rating {
            id,
            article_id: input_rating.article_id,
            date: naive_date_time,
            rating: input_rating.rating,
        };
        diesel::insert_into(ratings::table)
            .values(&rating)
            .execute(connection)?;
        Ok(rating)
    }
}
