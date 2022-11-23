use aline_api::{api, model::account, model::app};
use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    init_db().unwrap();

    let app = Router::new()
        .route("/chains", get(api::chains))
        .route("/networks", get(api::networks))
        .route("/app", post(api::create_app))
        .route("/apps", get(api::get_apps));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn init_db() -> Result<()> {
    account::Account::init_db()?;
    app::App::init_db()?;
    Ok(())
}
