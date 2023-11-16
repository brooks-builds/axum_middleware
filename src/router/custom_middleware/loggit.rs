use axum::{http::Request, middleware::Next, response::Response, Extension};

pub async fn loggit<B>(
    Extension(message): Extension<&'static str>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    tracing::info!("logging before the route handler runs");
    tracing::info!("Now the message is {message}");

    let response = next.run(request).await;

    tracing::info!("logging after the route handler runs");

    response
}
