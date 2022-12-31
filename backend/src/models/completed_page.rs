use serde::Serialize;

#[derive(Debug, Serialize, Clone, Queryable)]
#[diesel(table_name = completed_pages)]
pub struct CompletedPage {
    pub id: i32,
    pub user_id: i32,
    pub article_id: i32,
}
