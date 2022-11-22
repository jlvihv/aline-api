use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

const ETHEREUM_HTTP: &str = "http://localhost/http_node";
const ETHEREUM_WS: &str = "ws://localhost/ws_node";

#[derive(Deserialize, Serialize)]
pub struct Response {
    code: u16,
    message: String,
    result: serde_json::Value,
}

impl Response {
    pub fn new(code: u16, message: String, result: serde_json::Value) -> Self {
        Self {
            code,
            message,
            result,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Chain {
    pub name: String,
    pub http_address: String,
    pub websocket_address: String,
}

impl Chain {
    pub fn new(name: &str, http_address: &str, websocket_address: &str) -> Self {
        Self {
            name: name.to_string(),
            http_address: http_address.to_string(),
            websocket_address: websocket_address.to_string(),
        }
    }
    pub fn get_all_default() -> Vec<Self> {
        vec![
            Chain::new("ethereum", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("bsc", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("polygon", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("avalanche", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("optimism", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("zksync", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("startware", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("near", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("aptos", ETHEREUM_HTTP, ETHEREUM_WS),
            Chain::new("sui", ETHEREUM_HTTP, ETHEREUM_WS),
        ]
    }

    pub fn get_all(account: Option<String>) -> Vec<Self> {
        if account.is_none() {
            return Chain::get_all_default();
        }
        let md5str = md5::compute(account.unwrap());
        vec![
            Chain::new(
                "ethereum",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "bsc",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "polygon",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "avalanche",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "optimism",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "zksync",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "startware",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "near",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "aptos",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
            Chain::new(
                "sui",
                format!("{}/{:x}", ETHEREUM_HTTP, md5str).as_str(),
                format!("{}/{:x}", ETHEREUM_WS, md5str).as_str(),
            ),
        ]
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChainsParams {
    account: Option<String>,
}

pub async fn chains(Query(params): Query<ChainsParams>) -> impl IntoResponse {
    match serde_json::to_value(Chain::get_all(params.account)) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new(200, "ok".to_string(), result)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(500, e.to_string(), serde_json::Value::Null)),
        ),
    }
}

pub async fn register() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(Response::new(
            200,
            "ok".to_string(),
            serde_json::Value::Null,
        )),
    )
}
