use serde::Serialize;
use crate::handlers::reports::InputReport;
use crate::schema::reports;
use diesel::{PgConnection, QueryDsl};
use crate::diesel::RunQueryDsl;
use crate::diesel::ExpressionMethods;
use rand::Rng;

#[derive(Debug, Serialize, Clone, Queryable, Insertable)]
#[diesel(table_name = reports)]
pub struct Report {
    pub id: i32,
    pub article_id: i32,
    pub report_cat: String,
    pub description: String,
}
impl Report {
    pub fn create(connection: &mut PgConnection, report: &InputReport) -> Result<Report, diesel::result::Error>{
        let mut rng = rand::thread_rng();
        let id = rng.gen::<i32>();
        let report = Report{id, article_id: report.article_id, report_cat: report.report_cat.to_owned(), description: report.description.to_owned() };
        diesel::insert_into(reports::table)
            .values(&report)
            .execute(connection)?;
        Ok(report)
    }
    pub fn get_all(connection: &mut PgConnection) -> Result<Vec<Report>, diesel::result::Error>{
        let all_reports = reports::table.load::<Report>(connection)?;
        Ok(all_reports)
    }

    pub fn get_article_reports(connection: &mut PgConnection, a_id: i32) -> Result<Vec<Report>, diesel::result::Error>{
        let query = reports::table.into_boxed();
        let query = query.filter(reports::article_id.eq(a_id));
        let results = query.load::<Report>(connection)?;
        println!("Report: {:?}", results);
        Ok(results)
    }
}
