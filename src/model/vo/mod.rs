use anyhow::Result;
use serde::Serialize;

use crate::handler::Json;

pub type Resp<T> = Json<RespVO<T>>;

#[derive(Debug, Serialize, Clone)]
pub struct RespVO<T> {
    pub code: i64,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> From<Result<T>> for RespVO<T>
where
    T: Serialize,
{
    fn from(item: Result<T>) -> Self {
        match item {
            Ok(data) => RespVO::success(data),
            Err(e) => RespVO::fail(1, e.to_string()),
        }
    }
}

impl<T> RespVO<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        RespVO {
            code: 0,
            msg: Some("success".to_string()),
            data: Some(data),
        }
    }

    pub fn fail(code: i64, e: String) -> Self {
        RespVO {
            code,
            msg: Some(e),
            data: None,
        }
    }
}
