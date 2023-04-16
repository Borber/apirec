use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetApiVO {
    pub api: String,
    pub count: Option<i64>,
}
