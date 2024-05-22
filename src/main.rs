use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::middleware;
use axum::routing::get_service;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;
pub use self::error::{Error};

mod error;
mod web;
mod model;

mod utils;
mod ctx;

#[tokio::main]
async fn main()->Result<(),Error> {

    let app = Router::new()
        .merge(web::routes_user::routes())
        .route_layer(middleware::from_fn(web::mw_auth::guard))
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error.as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
            "error":{
                "type": client_error.as_ref(),
                "req_uuid":uuid.to_string(),

            }
        });
            println!("   ->> client_error_body:{client_error_body}");
            (*status_code,Json(client_error_body)).into_response()
        });

    println!("   ->> server log line - {uuid} - Error: {service_error:?}");
    println!();
    error_response.unwrap_or(res)
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}