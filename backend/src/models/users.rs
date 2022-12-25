use crate::{schema::*, handlers::users::InputUser};
use chrono::NaiveDate;
use diesel::{PgConnection, QueryDsl};
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use serde::Serialize;
use actix_web::web;
use rand::Rng;
// use uuid::Uuid;



#[derive(Identifiable, Debug, Serialize, Clone, Queryable, Insertable)]
pub struct User {
    id: i32,
    id_session: i64,
    t_email: String,
    t_password: String,
    t_ip_address: String,
    d_visit_first: NaiveDate,
}

impl User {
    pub fn get_all(connection: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error>{
        let all_users = users::table.load::<User>(connection)?;
        Ok(all_users)
    }

    pub fn get_by_email(connection: &mut PgConnection, email: &str) -> Result<User, diesel::result::Error>{
        let query = users::table.into_boxed();
        let query = query.filter(users::t_email.eq(email));
        let results = query.load::<User>(connection)?;
        println!("User(s): {:?}", results);
        if let Some(user) = results.into_iter().next() {
            Ok(user)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    }

    pub fn create(connection: &mut PgConnection, user: web::Json<InputUser>) -> Result<User, diesel::result::Error>{
        let naive_date_time = chrono::Local::now().date_naive();
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        let new_user = User {
            id,
            id_session: 0,
            t_email: user.email.to_owned(),
            t_password: user.password.to_owned(),
            t_ip_address: "0.0.0.0".to_string(),
            d_visit_first: naive_date_time,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(connection)?;
        Ok(new_user)
    }
}
