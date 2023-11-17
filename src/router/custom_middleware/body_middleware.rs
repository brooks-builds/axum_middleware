use axum::{middleware::Next, response::Response};
use hyper::{Body, Request, StatusCode};
use serde::{Deserialize, Serialize};

pub async fn body_middleware(
    request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();

    let bytes = hyper::body::to_bytes(body).await.map_err(|error| {
        tracing::error!("Error getting bytes: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let raw_quote = std::str::from_utf8(&bytes).map_err(|error| {
        tracing::error!("Error converting bytes to &str: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let quote = serde_json::from_str::<Quote>(raw_quote).map_err(|error| {
        tracing::error!("Error converting raw body to quote: {error}");
        StatusCode::BAD_REQUEST
    })?;

    tracing::info!("Quote: {quote:?}");

    let body = Body::from(bytes);
    let request = Request::from_parts(parts, body);

    Ok(next.run(request).await)
}

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    quote: String,
    attribution: String,
}
