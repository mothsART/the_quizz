use schema::questions;

#[derive(Serialize, Debug, Identifiable, Queryable)]
pub struct Question {
    pub id: i32,
    pub entitled: String,
    pub explanation: Option<String>,
    pub source: Option<String>,
    pub response: bool,
}
