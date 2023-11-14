use axum_middleware::App;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    let port = dotenv!("PORT")
        .parse()
        .expect("PORT environment variable must be a valid port");
    let app = App::new(port);

    app.run().await.expect("Server shutting down with error");
}
