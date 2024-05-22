use crate::ctx::Ctx;
use crate::model::{ModelController, User};
use crate::{Error, utils};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::RequestPartsExt;
use axum::response::Response;
use axum_extra::headers::{Authorization, HeaderMapExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::utils::jwt::{decode_jwt};

pub async fn guard(mut req: Request<Body>, next: Next) -> Result<Response,Error> {

    let token = req.headers().typed_get::<Authorization<Bearer>>()
        .ok_or(Error::AuthFailTokenWrongFormat)?.token().to_owned();

    let claim = decode_jwt(token)
        .map_err(|err| Error::AuthFailCtxNotInRequestExt )?.claims;

    let user=User{
        id:1,
        email:"admin".to_string(),
        surname:"admin".to_string(),
        name:"admin".to_string()
    }; //TODO: Get real data from DB

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self,Error> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::AuthFailTokenWrongFormat)?;

        let secret = (*utils::constants::TOKEN).clone();
        let token_data = decode::<Ctx>(bearer.token(), &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
            .map_err(|_| Error::AuthFailTokenWrongFormat)?;

        Ok(token_data.claims)
    }
}


