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
    app_id_index: u32,
}

impl Account {
    pub fn new(address: &str) -> Result<Self> {
        let a = Self {
            address: address.to_string(),
            created_at: Local::now().to_string(),
            app_id_index: 0,
        };
        a.save()?;
        Ok(a)
    }

    pub fn get(address: &str) -> Result<Self> {
        let conn = Connection::open("db.sqlite")?;
        let mut stmt = conn.prepare(
            "SELECT address, created_at, app_id_index FROM account WHERE address = :address;",
        )?;
        let result = stmt.query_map(&[(":address", &address)], |row| {
            Ok(Account {
                address: row.get(0)?,
                created_at: row.get(1)?,
                app_id_index: row.get(2)?,
            })
        });
        match result {
            Ok(mut rows) => match rows.next() {
                Some(row) => Ok(row?),
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
        self.save()?;
        Ok(app)
    }

    pub fn delete_app(&self, id: &str) -> Result<()> {
        let conn = Connection::open("db.sqlite")?;
        let n = conn.execute(
            "DELETE FROM app WHERE account = :account AND id = :id;",
            &[(":account", &self.address), (":id", &id.to_string())],
        )?;
        if n == 0 {
            Err(anyhow!("app not found"))
        } else {
            Ok(())
        }
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
        let conn = Connection::open("db.sqlite")?;
        conn.execute(
            "INSERT OR REPLACE INTO account (address, created_at, app_id_index) VALUES (?1, ?2, ?3)",
            params![self.address, self.created_at, self.app_id_index],
        )?;
        Ok(())
    }

    pub fn get_apps_total(&self) -> Result<u32> {
        let conn = Connection::open("db.sqlite")?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM app WHERE account = :account;")?;
        let result = stmt.query_map(&[(":account", &self.address)], |row| row.get(0));
        match result {
            Ok(mut rows) => match rows.next() {
                Some(row) => Ok(row?),
                None => Ok(0),
            },
            Err(_) => Ok(0),
        }
    }

    pub fn get_apps(&self, page: u32, size: u32) -> Result<Vec<App>> {
        let offset = (page - 1) * size;
        let conn = Connection::open("db.sqlite")?;
        let mut stmt =
            conn.prepare("SELECT * FROM app WHERE account = :address LIMIT :offset, :size;")?;
        let result = stmt.query_map(
            &[
                (":address", &self.address),
                (":offset", &offset.to_string()),
                (":size", &size.to_string()),
            ],
            |row| {
                let mut a = App {
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
                    http_link: row.get(10)?,
                    websocket_link: row.get(11)?,
                    ..App::default()
                };
                a.generate_code_example();
                Ok(a)
            },
        );
        match result {
            Ok(rows) => {
                let mut apps = vec![];
                for row in rows {
                    let row = match row {
                        Ok(row) => row,
                        Err(e) => {
                            tracing::error!("error while getting app: {}", e);
                            continue;
                        }
                    };
                    apps.push(row);
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
