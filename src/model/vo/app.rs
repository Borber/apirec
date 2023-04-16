use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetAppVO {
    pub total: i64,
    pub apis: HashMap<String, i64>,
}
