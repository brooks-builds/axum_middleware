use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};

use crate::app_state::AppState;

pub async fn log_state<B>(
    state: State<AppState>,
    Extension(message): Extension<&'static str>,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    tracing::info!("state: {state:?}");
    tracing::info!("Extension message is {message}");

    let Some(message_extension) = request.extensions_mut().get_mut::<&'static str>() else {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    *message_extension = "hello from log_state";

    let response = next.run(request).await;

    response
}
