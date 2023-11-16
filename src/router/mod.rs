mod custom_middleware;
mod quotes;

use self::{
    custom_middleware::{handling_body::handling_body, log_state, loggit::loggit},
    quotes::create_quotes_router,
};
use crate::app_state::AppState;
use axum::{middleware, Extension, Router};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .nest("/quotes", create_quotes_router())
        .route_layer(middleware::from_fn(handling_body))
        // Load in the middleware for all routes, regardless if they match a path
        .layer(middleware::from_fn(loggit))
        // This middleware will have access to State
        // It will also run before the middleware on the line above.
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            log_state::log_state,
        ))
        .layer(Extension("Hello from main router"))
        .with_state(app_state)
}
