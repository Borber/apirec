use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
    response::IntoResponse,
};
use serde::Serialize;

use crate::model::vo::RespVO;

pub struct Json<T>(pub T);

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        axum::Json::<T>(self.0).into_response()
    }
}

#[async_trait]
impl<S, B, T> FromRequest<S, B> for Json<T>
where
    axum::Json<T>: FromRequest<S, B, Rejection = JsonRejection>,
    S: Send + Sync,
    B: Send + 'static,
{
    type Rejection = axum::Json<RespVO<()>>;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            // convert the error from `axum::Json` into whatever we want
            Err(rejection) => {
                let msg = match rejection {
                    JsonRejection::JsonDataError(_) => "Json data error",
                    JsonRejection::JsonSyntaxError(_) => "Json syntax error",
                    JsonRejection::MissingJsonContentType(_) => "Missing json content type",
                    _ => "Unknown error",
                };
                Err(axum::Json(RespVO::fail(
                    1,
                    format!("{msg}: {}", rejection.body_text()),
                )))
            }
        }
    }
}
