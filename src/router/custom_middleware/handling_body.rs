use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use hyper::{body::Body, StatusCode};
use serde::{Deserialize, Serialize};

pub async fn handling_body(
    request: Request<Body>,
    next: Next<Body>,
) -> Result<Response, StatusCode>
where
{
    let (parts, body) = request.into_parts();

    let bytes = hyper::body::to_bytes(body).await.map_err(|error| {
        tracing::error!("Error extracting bytes from body: {error}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let quote_string = std::str::from_utf8(&bytes).map_err(|error| {
        tracing::error!("handling body middleware has error: {error}");
        StatusCode::BAD_REQUEST
    })?;

    let quote = serde_json::from_str(quote_string).map_err(|error| {
        tracing::error!("Error converting body to json: {error}");
        StatusCode::BAD_REQUEST
    })?;

    tracing::info!("quote in middleware is {quote:?}");

    let body = Body::from(bytes);
    let request = Request::from_parts(parts, body);
    let response = next.run(request).await;

    Ok(response)
}

#[derive(Deserialize, Serialize, Debug)]
struct Quote {
    quote: String,
    attribution: String,
}
