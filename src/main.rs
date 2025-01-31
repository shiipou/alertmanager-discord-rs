mod models;
mod handlers;
mod discord;

use axum::{routing::post, Router};
use log::info;
use env_logger;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let state = discord::initialize_discord().await;

    let app = Router::new()
        .route("/webhook", post(handlers::webhook::handle_webhook))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    info!("Server running on port 4000");
    axum::serve(listener, app).await.unwrap();
}
