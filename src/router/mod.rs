mod quotes;

use axum::Router;

use self::quotes::create_quotes_router;

pub fn create_router() -> Router {
    Router::new().nest("/quotes", create_quotes_router())
}
