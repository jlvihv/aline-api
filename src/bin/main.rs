use aline_api::{api, model::account, model::app};
use anyhow::Result;
use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let pool = conn_db().await;

    init_db().unwrap();

    let app = Router::new()
        .layer(ServiceBuilder::new().layer(AddExtensionLayer::new(api::ApiContext { db: pool })))
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

fn db_url() -> Result<String> {
    let user = std::env::var("MYSQL_USER")?;
    let password = std::env::var("MYSQL_PASSWORD")?;
    let host = std::env::var("MYSQL_HOST")?;
    let port = std::env::var("MYSQL_PORT")?;
    let database = std::env::var("MYSQL_DATABASE")?;

    Ok(format!(
        "mysql://{}:{}@{}:{}/{}",
        user, password, host, port, database
    ))
}

async fn conn_db() -> Pool<MySql> {
    let url = db_url().unwrap();
    tracing::info!("Connecting to database: {}", url);
    MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&url)
        .await
        .unwrap()
}
