use axum::{
    routing::{delete, get, post},
    Router,
};
use node_service::{api, model::db};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    db::conn_db().await.expect("Failed to connect to database");

    let app = Router::new()
        .route("/chains", get(api::chains))
        .route("/networks/:chain", get(api::networks))
        .route("/apps/:account", get(api::get_apps))
        .route("/app", post(api::create_app))
        .route("/app/:account/:app_id", delete(api::delete_app));

    let addr = SocketAddr::from(([0, 0, 0, 0], 9911));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
