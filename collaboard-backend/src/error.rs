use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;


#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Validation error
    BadRequest,

    // -- DB errors.
    FailToGetPool,
    FailInsertDB,
    FailDeleteDB,
    FailUpdateDB,
    FailCreatingFile,

    // -- Auth errors.
    PermissionDenied,
    TokenEncodingFail,
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,
    // -- Model errors.
    UserNotFound,
    BoardNotFound,
    InvitationNotFound,
    InvitationExpired
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, client_error) = self.client_status_and_error();
        let body = serde_json::json!({
            "error": client_error.as_ref(),
        });
        (status, axum::Json(body)).into_response()
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::FailToGetPool |
            Self::FailInsertDB |
            Self::FailDeleteDB |
            Self::FailUpdateDB |
            Self::FailCreatingFile => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),


            Self::UserNotFound |
            Self::InvitationNotFound |
            Self::BoardNotFound => (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMS),

            Self::TokenEncodingFail |
            Self::AuthFailNoAuthTokenCookie |
            Self::AuthFailTokenWrongFormat |
            Self::AuthFailCtxNotInRequestExt =>(StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            Self::PermissionDenied=>(StatusCode::FORBIDDEN,ClientError::NO_AUTH),
            Self::InvitationExpired |
            Self::BadRequest=>(StatusCode::BAD_REQUEST,ClientError::INVALID_PARAMS),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR
            )
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    UNAUTHORIZED,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}