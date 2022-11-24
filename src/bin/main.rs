use aline_api::{api, model::account, model::app};
use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    init_db().unwrap();

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

fn init_db() -> Result<()> {
    account::Account::init_db()?;
    app::App::init_db()?;
    Ok(())
}
