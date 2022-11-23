use anyhow::{anyhow, Result};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use super::chain::{self, ChainEnum, NetworkEnum};

#[derive(Deserialize, Serialize, Debug, Clone)]
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
            api_key: "".to_string(),
            today_requests: 0,
            total_requests: 0,
            created_at: Local::now().to_string(),
        };
        app.generate_key();
        app.save()?;
        Ok(app)
    }

    fn generate_key(&mut self) {
        self.api_key = format!(
            "{:x}",
            md5::compute(format!("{}-{}-{}", self.account, self.id, self.created_at))
        );
    }

    pub fn init_db() -> Result<()> {
        let conn = Connection::open("db.sqlite")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app (
                account TEXT NOT NULL,
                id INTEGER NOT NULL,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                chain TEXT NOT NULL,
                network TEXT NOT NULL,
                api_key TEXT NOT NULL,
                today_requests INTEGER NOT NULL,
                total_requests INTEGER NOT NULL,
                created_at TEXT NOT NULL,
                PRIMARY KEY (account, id)
            );",
            params![],
        )?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let conn = Connection::open("db.sqlite")?;
        conn.execute(
            "INSERT INTO app (account, id, name, description, chain, network, api_key, today_requests, total_requests, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);",
            params![
                self.account,
                self.id,
                self.name,
                self.description,
                self.chain,
                self.network,
                self.api_key,
                self.today_requests,
                self.total_requests,
                self.created_at
            ],
        )?;
        Ok(())
    }

    pub fn get(account: &str, id: u32) -> Result<Self> {
        let conn = Connection::open("db.sqlite")?;
        let mut stmt = conn.prepare("SELECT * FROM app WHERE account = ?1 AND id = ?2;")?;
        let mut rows = stmt.query(params![account, id])?;
        match rows.next() {
            Ok(Some(row)) => {
                Ok(Self {
                    account: row.get(0)?,
                    id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    chain: row.get(4)?,
                    network: row.get(5)?,
                    api_key: row.get(6)?,
                    today_requests: row.get(7)?,
                    total_requests: row.get(8)?,
                    created_at: row.get(9)?,
                })
            }
            Ok(None) => Err(anyhow!("App not found")),
            Err(e) => Err(anyhow!(e)),
        }
    }
}
