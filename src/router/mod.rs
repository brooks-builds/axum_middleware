mod custom_middleware;
mod quotes;

use self::{
    custom_middleware::{accessing_state::accessing_state, hello_world::hello_world},
    quotes::create_quotes_router,
};
use crate::app_state::AppState;
use axum::{middleware, Extension, Router};
use custom_middleware::hello_world;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .nest("/quotes", create_quotes_router())
        .route_layer(middleware::from_fn(hello_world))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            accessing_state,
        ))
        .layer(Extension("Hello from main router"))
        .with_state(app_state)
}
