use std::net::SocketAddr;

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::model::{
    account::Account,
    chain::{Chain, ChainEnum, NetworkEnum},
};

fn get_listen_port() -> u16 {
    let port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT must be set");
    port.parse().expect("LISTEN_PORT must be a number")
}

pub async fn serve() {
    let addr = SocketAddr::from(([0, 0, 0, 0], get_listen_port()));
    println!("listening on {}", addr);
    tracing::info!("listening on {}", addr);

    let app = Router::new()
        .route("/chains", get(chains))
        .route("/networks/:chain", get(networks))
        .route("/apps/:account", get(get_apps))
        .route("/app", post(create_app))
        .route("/app/:account/:app_id", delete(delete_app));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Serialize)]
pub struct Response {
    message: String,
    result: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pagination: Option<Pagination>,
}

#[derive(Deserialize, Serialize)]
pub struct Pagination {
    page: Option<i64>,
    size: Option<i64>,
    total: Option<i64>,
}

impl Pagination {
    pub fn new(page: i64, size: i64, total: i64) -> Self {
        Self {
            page: Some(page),
            size: Some(size),
            total: Some(total),
        }
    }
}

impl Response {
    pub fn new(message: String, result: serde_json::Value, pagination: Option<Pagination>) -> Self {
        Self {
            message,
            result,
            pagination,
        }
    }
}

pub async fn chains() -> impl IntoResponse {
    match serde_json::to_value(ChainEnum::get_all()) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new("ok".to_string(), result, None)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
        ),
    }
}

pub async fn networks(Path(chain): Path<String>) -> impl IntoResponse {
    let Ok(chain) = chain.parse::<ChainEnum>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("chain invalid".to_string(), serde_json::Value::Null, None)));
    };
    match serde_json::to_value(Chain::new(chain).networks) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new("ok".to_string(), result, None)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
        ),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateApp {
    pub name: String,
    pub description: String,
    pub chain: String,
    pub network: String,
    pub account: String,
}

pub async fn create_app(Json(payload): Json<CreateApp>) -> impl IntoResponse {
    let mut user = match Account::get(&payload.account).await {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
            )
        }
    };
    let Ok(chain) = payload.chain.parse::<ChainEnum>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("invaild parameters".to_string(), serde_json::Value::Null, None)));
    };
    let Ok(network) = payload.network.parse::<NetworkEnum>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("invaild parameters".to_string(), serde_json::Value::Null, None)));
    };
    let app = match user
        .create_app(&payload.name, &payload.description, chain, network)
        .await
    {
        Ok(app) => app,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response::new(
                    format!("create app failed, {}", e),
                    serde_json::Value::Null,
                    None,
                )),
            )
        }
    };
    match serde_json::to_value(app) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new("ok".to_string(), result, None)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
        ),
    }
}

pub async fn get_apps(
    Path(account): Path<String>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    let Ok(user) = Account::get(&account).await else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("account invalid".to_string(), serde_json::Value::Null, None)));
    };
    let size = pagination.size.unwrap_or(10);
    let page = pagination.page.unwrap_or(1);

    let apps = match user.get_apps(page, size).await {
        Ok(apps) => apps,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
            )
        }
    };
    let total = match user.get_apps_total().await {
        Ok(total) => total,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
            )
        }
    };
    match serde_json::to_value(apps) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new(
                "ok".to_string(),
                result,
                Some(Pagination::new(page, size, total)),
            )),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
        ),
    }
}

pub async fn delete_app(Path((account, app_id)): Path<(String, String)>) -> impl IntoResponse {
    let Ok(user) = Account::get(&account).await else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("account invalid".to_string(), serde_json::Value::Null, None)));
    };
    match user.delete_app(&app_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(Response::new(
                "ok".to_string(),
                serde_json::Value::Null,
                None,
            )),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(Response::new(e.to_string(), serde_json::Value::Null, None)),
        ),
    }
}
