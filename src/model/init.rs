use crate::model::{db, log_parse};

pub async fn init() {
    dotenvy::dotenv().ok();

    let file_appender = tracing_appender::rolling::daily("log", "node-service.log");
    tracing_subscriber::fmt()
        .with_writer(file_appender)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .init();
    tracing::info!("log init finished");

    db::init().await.expect("Failed to connect to database");
    log_parse::cache::init().await.expect("Failed to cache log");
}
