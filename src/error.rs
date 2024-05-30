use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;


#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {

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
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INFO_RES");
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);
        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::TokenEncodingFail |
            Self::FailToGetPool |
            Self::FailInsertDB |
            Self::FailDeleteDB |
            Self::FailUpdateDB |
            Self::FailCreatingFile => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::SERVICE_ERROR),

            Self::UserNotFound |
            Self::InvitationNotFound |
            Self::BoardNotFound => (StatusCode::NOT_FOUND, ClientError::INVALID_PARAMS),
            Self::AuthFailNoAuthTokenCookie |
            Self::AuthFailTokenWrongFormat |
            Self::AuthFailCtxNotInRequestExt =>(StatusCode::UNAUTHORIZED, ClientError::NO_AUTH),

            Self::PermissionDenied=>(StatusCode::FORBIDDEN,ClientError::NO_AUTH),

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