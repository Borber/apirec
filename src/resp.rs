use anyhow::Result;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Resp<T> {
    pub code: i64,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> IntoResponse for Resp<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json::<Resp<T>>(self).into_response()
    }
}

impl<T> From<Result<T>> for Resp<T>
where
    T: Serialize,
{
    fn from(item: Result<T>) -> Self {
        match item {
            Ok(data) => Resp::success(data),
            Err(e) => Resp::fail((1, &e.to_string())),
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

    pub fn fail(typ: (i64, &str)) -> Self {
        Resp {
            code: typ.0,
            msg: Some(typ.1.to_owned()),
            data: None,
        }
    }
}
