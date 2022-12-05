use std::str::FromStr;

use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn init() -> Result<(), Pool<Postgres>> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("Connecting to database: {}", url);
    let mut connection_options = match PgConnectOptions::from_str(url.as_str()) {
        Ok(connection_options) => connection_options,
        Err(e) => {
            tracing::error!("Failed to parse DATABASE_URL: {}", e);
            return set_pool_with_log(&url).await;
        }
    };
    // 完全禁用 sqlx 日志，不然太冗长了
    connection_options.disable_statement_logging();
    POOL.set({
        PgPoolOptions::new()
            .max_connections(50)
            .connect_with(connection_options)
            .await
            .expect("Failed to connect to database")
    })
}

async fn set_pool_with_log(url: &str) -> Result<(), Pool<Postgres>> {
    POOL.set({
        PgPoolOptions::new()
            .max_connections(50)
            .connect(url)
            .await
            .expect("Failed to connect to database")
    })
}

pub fn get_pool() -> Result<Pool<Postgres>> {
    match POOL.get() {
        Some(pool) => Ok(pool.clone()),
        None => Err(anyhow!("Database connection pool not initialized")),
    }
}
