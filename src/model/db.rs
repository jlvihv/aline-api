use anyhow::{anyhow, Result};
use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn init() -> Result<(), Pool<Postgres>> {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    tracing::info!("Connecting to database: {}", url);
    POOL.set({
        PgPoolOptions::new()
            .max_connections(50)
            .connect(&url)
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
