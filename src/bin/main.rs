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
    init().await;
    serve().await;
}

fn get_listen_port() -> u16 {
    let port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set");
    port.parse().expect("LISTEN_PORT must be a number")
}

async fn init() {
    dotenvy::dotenv().ok();

    let file_appender = tracing_appender::rolling::daily("log", "node-service.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .init();
    tracing::info!("log init finished");

    db::init().await.expect("Failed to connect to database");
    log_parse::cache::init().await.expect("Failed to cache log");
}

async fn serve() {
    let addr = SocketAddr::from(([0, 0, 0, 0], get_listen_port()));
    println!("listening on {}", addr);

    let app = Router::new()
        .route("/chains", get(api::chains))
        .route("/networks/:chain", get(api::networks))
        .route("/apps/:account", get(api::get_apps))
        .route("/app", post(api::create_app))
        .route("/app/:account/:app_id", delete(api::delete_app));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
