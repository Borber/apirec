use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetAppVO {
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<ApiCount>>,
}

#[derive(Debug, Serialize)]
pub struct ApiCount {
    pub api: String,
    pub count: i64,
}
