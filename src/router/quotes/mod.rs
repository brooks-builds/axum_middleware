mod create;

use axum::{routing::post, Router};
use create::create_quote;

pub fn create_quotes_router() -> Router {
    Router::new().route("/", post(create_quote))
}
