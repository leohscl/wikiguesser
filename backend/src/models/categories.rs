use serde::Serialize;

#[derive(Debug, Serialize, Clone, Queryable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub article_id: i32,
    pub category: String,
}
