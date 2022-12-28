use anyhow::{anyhow, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};

use super::{
    chain::{self, ChainEnum, NetworkEnum},
    code_examples, db, log_parse,
};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct App {
    pub account: String,
    pub id: i32,
    pub name: String,
    pub description: String,
    pub chain: String,
    pub network: String,
    pub api_key: String,
    pub created_at: String,
    pub http_link: String,
    pub websocket_link: String,
    pub code_examples: code_examples::CodeExample,
    pub total_requests_today: i32,
    pub dayly_requests_7days: Vec<i32>,
}

impl App {
    pub async fn new(
        account: &str,
        id: i32,
        name: &str,
        description: &str,
        chain: ChainEnum,
        network: NetworkEnum,
    ) -> Result<Self> {
        let ch = chain::Chain::new(chain.clone());
        if !ch.have_network(&network.to_string()) {
            return Err(anyhow!("Network not found"));
        }
        let mut app = Self {
            account: account.to_string(),
            id,
            name: name.to_string(),
            description: description.to_string(),
            chain: chain.to_string(),
            network: network.to_string(),
            created_at: Local::now().to_string(),
            ..Default::default()
        };
        app.generate_key()?;
        app.save().await?;
        app.generate_code_example(chain);
        Ok(app)
    }

    pub async fn delete(account: &str, id: i32) -> Result<()> {
        let n = sqlx::query!(
            "DELETE FROM apps WHERE account = $1 AND id = $2;",
            account,
            id
        )
        .execute(&db::get_pool()?)
        .await?;
        if 0 == n.rows_affected() {
            Err(anyhow!("App not found"))
        } else {
            Ok(())
        }
    }

    pub async fn get_total(account: &str) -> Result<i64> {
        sqlx::query!(
            "SELECT COUNT(*) as total FROM apps WHERE account = $1",
            account
        )
        .fetch_one(&db::get_pool()?)
        .await?
        .total
        .ok_or_else(|| anyhow!("Failed to get total"))
    }

    pub async fn get_with_page(account: &str, page: i64, size: i64) -> Result<Vec<App>> {
        if page <= 0 {
            return Err(anyhow!("Page must be greater than 0"));
        }
        let offset = (page - 1) * size;
        let apps = sqlx::query!(
            "SELECT
                account, id, name, description, chain, network, api_key,
                created_at, http_link, websocket_link
            FROM apps
            WHERE
                account = $1
            ORDER BY
                id DESC
            LIMIT $2
            OFFSET $3;",
            account,
            size,
            offset,
        )
        .fetch_all(&db::get_pool()?)
        .await?;
        let mut result: Vec<App> = vec![];
        for a in apps {
            let mut app = App {
                account: a.account,
                id: a.id,
                name: a.name,
                description: a.description,
                chain: a.chain.clone(),
                network: a.network,
                api_key: a.api_key,
                created_at: a.created_at,
                http_link: a.http_link,
                websocket_link: a.websocket_link,
                ..Default::default()
            };
            app.generate_code_example(a.chain.parse().unwrap_or(ChainEnum::Ethereum));
            app.get_total_requests_today().await?;
            app.get_dayly_requests_7days().await?;
            result.push(app);
        }
        Ok(result)
    }

    async fn save(&self) -> Result<()> {
        sqlx::query!(
            "INSERT INTO apps (
                account, id, name, description,
                chain, network, api_key,
                created_at, http_link, websocket_link
            ) VALUES (
                $1, $2, $3, $4,
                $5, $6, $7,
                $8, $9, $10
            );",
            self.account,
            self.id,
            self.name,
            self.description,
            self.chain,
            self.network,
            self.api_key,
            self.created_at,
            self.http_link,
            self.websocket_link,
        )
        .execute(&db::get_pool()?)
        .await?;
        Ok(())
    }

    fn generate_key(&mut self) -> Result<()> {
        self.api_key = format!(
            "{:x}",
            md5::compute(format!("{}-{}", self.account, self.id))
        );
        let chain = match self.chain.parse::<chain::ChainEnum>() {
            Ok(c) => c,
            Err(_) => return Err(anyhow!("Chain not found")),
        };
        let chain = chain::Chain::new(chain);
        self.http_link = format!("{}/{}", chain.http_address, self.api_key);
        self.websocket_link = format!("{}/{}", chain.websocket_address, self.api_key);
        Ok(())
    }

    pub fn generate_code_example(&mut self, chain_type: ChainEnum) {
        self.code_examples = code_examples::get_code_example(&self.http_link, chain_type);
    }

    async fn get_total_requests_today(&mut self) -> Result<()> {
        let log = match log_parse::query::QueryLog::query_today(&self.api_key).await {
            Ok(l) => l,
            Err(_) => {
                tracing::error!("Failed to get total requests today");
                return Ok(());
            }
        };
        self.total_requests_today = log.result.len() as i32;
        Ok(())
    }

    async fn get_dayly_requests_7days(&mut self) -> Result<()> {
        let logs = match log_parse::query::QueryLog::query_7days(&self.api_key).await {
            Ok(l) => l,
            Err(_) => {
                tracing::error!("Failed to get dayly requests 7days");
                return Ok(());
            }
        };
        let mut result = Vec::new();
        logs.iter().for_each(|log| {
            result.push(log.result.len() as i32);
        });
        result.reverse();
        self.dayly_requests_7days = result;
        Ok(())
    }
}
