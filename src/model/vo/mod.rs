pub mod app;

use anyhow::Result;
use ntex::{
    http::{Response, StatusCode},
    web::{ErrorRenderer, HttpRequest, Responder},
};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Resp<T> {
    pub code: i64,
    pub msg: Option<String>,
    pub data: Option<T>,
}

// 自定义 ntex 的返回体, 以达到统一的目的
// Custom ntex return body to achieve the same purpose
impl<T, Err: ErrorRenderer> Responder<Err> for Resp<T>
where
    T: Serialize,
{
    async fn respond_to(self, _: &HttpRequest) -> Response {
        Response::build(StatusCode::OK)
            .content_type("application/json; charset=utf-8")
            .body(serde_json::to_string(&self).unwrap())
    }
}

impl<T> From<anyhow::Result<T>> for Resp<T>
where
    T: Serialize,
{
    fn from(item: Result<T>) -> Self {
        match item {
            Ok(data) => Resp::success(data),
            Err(e) => Resp::fail(1, e.to_string()),
        }
    }
}

impl<T> Resp<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        Resp {
            code: 0,
            msg: Some("success".to_string()),
            data: Some(data),
        }
    }

    pub fn fail(code: i64, e: String) -> Self {
        Resp {
            code,
            msg: Some(e),
            data: None,
        }
    }
}
