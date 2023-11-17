use axum::Extension;

pub async fn create_quote(
    Extension(message): Extension<&'static str>,
    Extension(port): Extension<u16>,
) -> &'static str {
    tracing::info!("Create quote route handler running");
    tracing::info!("{message}");
    tracing::info!("port is: {port}");
    "Hello from create quote route handler"
}
