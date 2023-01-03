use serde::Serialize;

#[derive(Debug, Serialize, Clone, Queryable)]
#[diesel(table_name = reports)]
pub struct Reports {
    pub id: i32,
    pub article_id: i32,
    pub report_cat: String,
    pub description: String,
}
