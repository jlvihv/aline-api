use node_service::{api, model::init};

#[tokio::main]
async fn main() {
    init::init().await;
    api::serve().await;
}
