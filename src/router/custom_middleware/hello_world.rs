use axum::{
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::Request;

pub async fn hello_world<B>(request: Request<B>, next: Next<B>) -> Response {
    tracing::info!("hello from hello world middleware");
    let response = next.run(request).await;
    tracing::info!("How did everything go?");

    response
    // StatusCode::BAD_REQUEST.into_response()
}
