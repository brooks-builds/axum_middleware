mod create;

use axum::{middleware, routing::post, Router};
use create::create_quote;

use crate::app_state::AppState;

use super::custom_middleware::body_middleware::body_middleware;

pub fn create_quotes_router() -> Router<AppState> {
    Router::new().route(
        "/",
        post(create_quote).route_layer(middleware::from_fn(body_middleware)),
    )
}
