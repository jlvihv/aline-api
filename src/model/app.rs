use anyhow::{anyhow, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};

use super::{
    chain::{self, ChainEnum, NetworkEnum},
    code_examples,
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
    pub fn new(
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
        app.save()?;
        app.generate_code_example();
        Ok(app)
    }

    fn generate_key(&mut self) -> Result<()> {
        self.api_key = format!(
            "{:x}",
            md5::compute(format!("{}-{}-{}", self.account, self.id, self.created_at))
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

    pub fn init_db() -> Result<()> {
        // let sql = std::env::var("SQL_CREATE_TABLE_APP")?;
        // let mut conn = db::get_connection()?;
        // if let Err(e) = conn.query_drop(sql) {
        //     tracing::error!("Error creating table app: {}", e);
        //     Err(anyhow!(e))
        // } else {
        //     Ok(())
        // }
        Ok(())
    }

    fn save(&self) -> Result<()> {
        // let mut conn = db::get_connection()?;
        // conn.exec_drop(
        //     "INSERT INTO app(
        //         account, id, name, description, chain, network, api_key, today_requests, total_requests, created_at, http_link, websocket_link
        //     ) VALUES (
        //         :account, :id, :name, :description, :chain, :network, :api_key, :today_requests, :total_requests, :created_at, :http_link, :websocket_link
        //     );",
        //     params! {
        //         "account" => &self.account,
        //         "id" => &self.id,
        //         "name" => &self.name,
        //         "description" => &self.description,
        //         "chain" => &self.chain,
        //         "network" => &self.network,
        //         "api_key" => &self.api_key,
        //         "today_requests" => &self.today_requests,
        //         "total_requests" => &self.total_requests,
        //         "created_at" => &self.created_at,
        //         "http_link" => &self.http_link,
        //         "websocket_link" => &self.websocket_link,
        //     },
        // )?;
        Ok(())
    }

    pub fn generate_code_example(&mut self) {
        self.code_examples = code_examples::get_code_example(&self.http_link);
    }

    pub fn get(account: &str, id: u32) -> Result<Self> {
        // let mut conn = db::get_connection()?;
        // conn.exec_first(
        //     "SELECT * FROM app WHERE account = :account AND id = :id;",
        //     params! {
        //         "account" => account,
        //         "id" => id,
        //     },
        // )?
        // .map(|row| {
        //     let (
        //         account,
        //         id,
        //         name,
        //         description,
        //         chain,
        //         network,
        //         api_key,
        //         today_requests,
        //         total_requests,
        //         created_at,
        //         http_link,
        //         websocket_link,
        //     ): (
        //         String,
        //         u32,
        //         String,
        //         String,
        //         String,
        //         String,
        //         String,
        //         u32,
        //         u32,
        //         String,
        //         String,
        //         String,
        //     ) = mysql::from_row(row);
        //     let mut app = Self {
        //         account,
        //         id,
        //         name,
        //         description,
        //         chain,
        //         network,
        //         api_key,
        //         today_requests,
        //         total_requests,
        //         created_at,
        //         http_link,
        //         websocket_link,
        //         ..Default::default()
        //     };
        //     app.generate_code_example();
        //     Ok(app)
        // })
        // .unwrap_or(Err(anyhow!("App not found")))

        Ok(Self::default())
    }
}
