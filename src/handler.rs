use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_macros::FromRequest;

use crate::resp::Resp;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(CustomRejection))]
pub struct Json<T>(pub T);

pub struct CustomRejection(Response);

impl From<JsonRejection> for CustomRejection {
    fn from(rejection: JsonRejection) -> Self {
        let response = (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::Json(Resp::<String>::fail(1010, &format!("{rejection:?}"))),
        )
            .into_response();

        CustomRejection(response)
    }
}

impl IntoResponse for CustomRejection {
    fn into_response(self) -> Response {
        self.0
    }
}
