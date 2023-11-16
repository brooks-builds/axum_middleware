mod create;

use axum::{routing::post, Router};
use create::create_quote;

use crate::app_state::AppState;

pub fn create_quotes_router() -> Router<AppState> {
    Router::new().route("/", post(create_quote))
}
