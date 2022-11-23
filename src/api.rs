use std::{fmt, str::FromStr};

use axum::{extract::Query, http::StatusCode, response::IntoResponse, Json};
use serde::{de, Deserialize, Deserializer, Serialize};

use crate::model::{
    account::Account,
    chain::{Chain, ChainEnum, NetworkEnum},
};

#[derive(Deserialize, Serialize)]
pub struct Response {
    message: String,
    result: serde_json::Value,
}

impl Response {
    pub fn new(message: String, result: serde_json::Value) -> Self {
        Self { message, result }
    }
}

pub async fn chains() -> impl IntoResponse {
    match serde_json::to_value(ChainEnum::get_all()) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new("ok".to_string(), result)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null)),
        ),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NetworksParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    chain: Option<String>,
}

pub async fn networks(Query(params): Query<NetworksParams>) -> impl IntoResponse {
    let Some(chain) = params.chain else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("chain required".to_string(), serde_json::Value::Null)));
    };
    let Ok(chain) = chain.parse::<ChainEnum>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("chain invalid".to_string(), serde_json::Value::Null)));
    };
    match serde_json::to_value(Chain::new(chain).networks) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new("ok".to_string(), result)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null)),
        ),
    }
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
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
    let Ok(mut user) = payload.account.parse::<Account>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("invaild parameters".to_string(), serde_json::Value::Null)));
    };
    let Ok(chain) = payload.chain.parse::<ChainEnum>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("invaild parameters".to_string(), serde_json::Value::Null)));
    };
    let Ok(network) = payload.network.parse::<NetworkEnum>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("invaild parameters".to_string(), serde_json::Value::Null)));
    };
    let app = match user.create_app(&payload.name, &payload.description, chain, network) {
        Ok(app) => app,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Response {
                    message: format!("create app failed, {}", e),
                    result: serde_json::Value::Null,
                }),
            )
        }
    };
    match serde_json::to_value(app) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new("ok".to_string(), result)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null)),
        ),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AppsParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    account: Option<String>,
}

pub async fn get_apps(Query(params): Query<AppsParams>) -> impl IntoResponse {
    let Some(account) = params.account else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("account required".to_string(), serde_json::Value::Null)));
    };
    let Ok(user) = account.parse::<Account>() else {
        return (StatusCode::BAD_REQUEST, Json(Response::new("account invalid".to_string(), serde_json::Value::Null)));
    };
    let apps = match user.get_apps() {
        Ok(apps) => apps,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(Response::new(e.to_string(), serde_json::Value::Null)),
            )
        }
    };
    match serde_json::to_value(apps) {
        Ok(result) => (
            StatusCode::OK,
            Json(Response::new("ok".to_string(), result)),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(Response::new(e.to_string(), serde_json::Value::Null)),
        ),
    }
}
