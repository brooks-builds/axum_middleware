use axum::{extract::State, middleware::Next, response::Response, Extension};
use hyper::Request;

use crate::app_state::AppState;

pub async fn accessing_state<B>(
    state: State<AppState>,
    Extension(message): Extension<&'static str>,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    tracing::info!("The state is: {:?}", &state);
    tracing::info!("The message is: {message}");

    let extensions = request.extensions_mut();
    if let Some(message) = extensions.get_mut::<&'static str>() {
        *message = "I have been changed";
    }

    extensions.insert(state.port);

    next.run(request).await
}
