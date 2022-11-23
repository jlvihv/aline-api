use anyhow::{anyhow, Result};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use chrono::prelude::*;

use super::{
    app::App,
    chain::{ChainEnum, NetworkEnum},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Account {
    pub address: String,
    pub created_at: String,
    pub apps: Vec<App>,
    app_id_index: u32,
}

impl Account {
    pub fn new(address: &str) -> Result<Self> {
        let a = Self {
            address: address.to_string(),
            created_at: Local::now().to_string(),
            apps: vec![],
            app_id_index: 0,
        };
        a.save()?;
        Ok(a)
    }

    pub fn get(address: &str) -> Result<Self> {
        let conn = Connection::open("db.sqlite").unwrap();
        let mut stmt = conn
            .prepare("SELECT address, created_at, app_id_index FROM account WHERE address = :address;")
            .unwrap();
        let result = stmt.query_map(&[(":address", &address)], |row| {
            Ok(Account {
                address: row.get(0)?,
                created_at: row.get(1)?,
                apps: vec![],
                app_id_index: row.get(2)?,
            })
        });
        match result {
            Ok(mut rows) => match rows.next() {
                Some(row) => Ok(row.unwrap()),
                None => Account::new(address),
            },
            Err(_) => Account::new(address),
        }
    }

    pub fn create_app(
        &mut self,
        name: &str,
        description: &str,
        chain: ChainEnum,
        network: NetworkEnum,
    ) -> Result<App> {
        let app = App::new(
            &self.address,
            self.app_id_index,
            name,
            description,
            chain,
            network,
        )?;
        self.app_id_index += 1;
        self.apps.push(app.clone());
        self.save()?;
        Ok(app)
    }

    pub fn init_db() -> Result<()> {
        let conn = Connection::open("db.sqlite")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS account(
            address TEXT PRIMARY KEY,
            created_at TEXT NOT NULL,
            app_id_index INTEGER NOT NULL
        )",
            (),
        )?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let conn = Connection::open("db.sqlite").unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO account (address, created_at, app_id_index) VALUES (?1, ?2, ?3)",
            params![self.address, self.created_at, self.app_id_index],
        )?;
        Ok(())
    }

    pub fn get_apps(&self) -> Result<Vec<App>> {
        let conn = Connection::open("db.sqlite")?;
        let mut stmt = conn.prepare("SELECT * FROM app WHERE account = :address;")?;
        let result = stmt.query_map(&[(":address", &self.address)], |row| {
            Ok(App {
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
        });
        match result {
            Ok(rows) => {
                let mut apps = vec![];
                for row in rows {
                    apps.push(row.unwrap());
                }
                Ok(apps)
            }
            Err(e) => Err(anyhow!(e)),
        }
    }
}

impl FromStr for Account {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::get(s)
    }
}
