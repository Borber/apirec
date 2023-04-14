use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetApiDTO {
    pub api: String,
    pub count: Option<i64>,
}
