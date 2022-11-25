use anyhow::{anyhow, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};

use super::{
    chain::{self, ChainEnum, NetworkEnum},
    code_examples, db,
};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct App {
    pub account: String,
    pub id: u32,
    pub name: String,
    pub description: String,
    pub chain: String,
    pub network: String,
    pub api_key: String,
    pub today_requests: u32,
    pub total_requests: u32,
    pub created_at: String,
    pub http_link: String,
    pub websocket_link: String,
    pub code_examples: code_examples::CodeExample,
}

impl App {
    pub async fn new(
        account: &str,
        id: u32,
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
            today_requests: 0,
            total_requests: 0,
            created_at: Local::now().to_string(),
            ..Default::default()
        };
        app.generate_key()?;
        app.save().await?;
        app.generate_code_example();
        Ok(app)
    }

    fn generate_key(&mut self) -> Result<()> {
        self.api_key = format!(
            "{:x}",
            md5::compute(format!("{}-{}", self.account, self.id))
        );
        let ch = match self.chain.parse::<chain::ChainEnum>() {
            Ok(c) => c,
            Err(_) => return Err(anyhow!("Chain not found")),
        };
        let ch = chain::Chain::new(ch);
        self.http_link = format!("{}/{}", ch.http_address, self.api_key);
        self.websocket_link = format!("{}/{}", ch.websocket_address, self.api_key);
        Ok(())
    }

    async fn save(&self) -> Result<()> {
        let pool = db::get_db_pool();
        sqlx::query!(
            "INSERT INTO app (
                account, id, name, description, chain, network, api_key, today_requests, total_requests, created_at, http_link, websocket_link
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
            );",
            self.account,
            self.id as i32,
            self.name,
            self.description,
            self.chain,
            self.network,
            self.api_key,
            self.today_requests as i32,
            self.total_requests as i32,
            self.created_at,
            self.http_link,
            self.websocket_link,
        )
        .execute(&pool).await?;
        Ok(())
    }

    pub fn generate_code_example(&mut self) {
        self.code_examples = code_examples::get_code_example(&self.http_link);
    }

    pub async fn get(account: &str, id: u32) -> Result<Self> {
        let pool = db::get_db_pool();
        let app = sqlx::query!(
            "SELECT * FROM app WHERE account = $1 AND id = $2;",
            account,
            id as i32,
        )
        .fetch_one(&pool)
        .await?;

        let app = Self {
            account: app.account,
            id: app.id as u32,
            name: app.name,
            description: app.description,
            chain: app.chain,
            network: app.network,
            api_key: app.api_key,
            today_requests: app.today_requests as u32,
            total_requests: app.total_requests as u32,
            created_at: app.created_at,
            http_link: app.http_link,
            websocket_link: app.websocket_link,
            ..Default::default()
        };

        Ok(app)
    }
}
