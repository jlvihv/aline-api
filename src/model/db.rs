use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

static POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn conn_db() -> Result<(), Pool<Postgres>> {
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

pub fn get_db_pool() -> Pool<Postgres> {
    POOL.get().expect("Database pool not initialized").clone()
}
