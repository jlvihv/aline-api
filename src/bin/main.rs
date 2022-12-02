use axum::{
    routing::{delete, get, post},
    Router,
};
use node_service::{
    api,
    model::{db, log_parse},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    db::init().await.expect("Failed to connect to database");
    log_parse::cache::init().await.expect("Failed to cache log");

    let app = Router::new()
        .route("/chains", get(api::chains))
        .route("/networks/:chain", get(api::networks))
        .route("/apps/:account", get(api::get_apps))
        .route("/app", post(api::create_app))
        .route("/app/:account/:app_id", delete(api::delete_app));

    let addr = SocketAddr::from(([0, 0, 0, 0], get_listen_port()));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_listen_port() -> u16 {
    let port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set");
    port.parse().expect("LISTEN_PORT must be a number")
}
