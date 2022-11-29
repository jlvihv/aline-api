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
        app.generate_code_example();
        Ok(app)
    }

    pub async fn get(account: &str, id: i32) -> Result<Self> {
        let app = sqlx::query!(
            "SELECT * FROM apps WHERE account = $1 AND id = $2;",
            account,
            id,
        )
        .fetch_one(&db::get_pool()?)
        .await?;

        Ok(Self {
            account: app.account,
            id: app.id,
            name: app.name,
            description: app.description,
            chain: app.chain,
            network: app.network,
            api_key: app.api_key,
            created_at: app.created_at,
            http_link: app.http_link,
            websocket_link: app.websocket_link,
            ..Default::default()
        })
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

    pub fn generate_code_example(&mut self) {
        self.code_examples = code_examples::get_code_example(&self.http_link);
    }
}
