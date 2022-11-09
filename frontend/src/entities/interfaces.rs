use serde::Deserialize;

#[derive(Deserialize)]
pub enum Status {
    Success,
    Error,
    Unknown,
}
