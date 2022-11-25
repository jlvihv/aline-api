use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

use chrono::prelude::*;

use super::{
    app::App,
    chain::{ChainEnum, NetworkEnum},
};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Account {
    pub address: String,
    pub created_at: String,
    app_id_index: u32,
}

impl Account {
    pub fn new(address: &str, pool: Pool<MySql>) -> Result<Self> {
        let a = Self {
            address: address.to_string(),
            created_at: Local::now().to_string(),
            app_id_index: 0,
        };
        a.save(pool)?;
        Ok(a)
    }

    pub fn get(address: &str, pool: Pool<MySql>) -> Result<Self> {
        //     let mut conn = db::get_connection()?;
        //     let result: Option<(String, String, u32)> = conn.exec_first(
        //         "SELECT address, created_at, app_id_index FROM account WHERE address = :address;",
        //         params! {
        //             "address" => address,
        //         },
        //     )?;
        //     if let Some(row) = result {
        //         tracing::debug!("Account found: {:?}", row);
        //         Ok(Self {
        //             address: row.0,
        //             created_at: row.1,
        //             app_id_index: row.2,
        //         })
        //     } else {
        //         Account::new(address)
        //     }
        Ok(Self::default())
    }

    pub fn create_app(
        &mut self,
        name: &str,
        description: &str,
        chain: ChainEnum,
        network: NetworkEnum,
        pool: &Pool<MySql>,
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
        self.save(pool)?;
        Ok(app)
    }

    pub fn delete_app(&self, id: &str) -> Result<()> {
        // let mut conn = db::get_connection()?;
        // let result = conn.exec_first(
        //     "DELETE FROM app WHERE account = :account AND id = :id;",
        //     params! {
        //         "account" => &self.address,
        //         "id" => id.parse::<u32>()?,
        //     },
        // )?.unwrap();

        Ok(())
    }

    pub fn init_db() -> Result<()> {
        //     let sql = std::env::var("SQL_CREATE_TABLE_ACCOUNT")?;
        //     let mut conn = db::get_connection()?;
        //     if let Err(e) = conn.query_drop(sql) {
        //         tracing::error!("Error creating table account: {}", e);
        //         Err(anyhow!(e))
        //     } else {
        //         Ok(())
        //     }
        Ok(())
    }

    fn save(&self, pool: Pool<MySql>) -> Result<()> {
        //     let mut conn = db::get_connection()?;
        //     if let Err(e) = conn.exec_drop(
        //         "INSERT INTO account (address, created_at, app_id_index) VALUES (:address, :created_at, :app_id_index)
        //         ON DUPLICATE KEY UPDATE app_id_index = :app_id_index;",
        //         params!{
        //             "address" => &self.address,
        //             "created_at" => &self.created_at,
        //             "app_id_index" => &self.app_id_index,
        //         },
        //     ) {
        //         tracing::error!("Error saving account: {}", e);
        //         Err(anyhow!(e))
        //     } else {
        //         tracing::debug!("Account saved: {:?}", self);
        //         Ok(())
        //     }
        Ok(())
    }

    pub fn get_apps_total(&self) -> Result<u32> {
        //     let mut conn = db::get_connection()?;
        //     let result: Option<u32> = conn.exec_first(
        //         "SELECT COUNT(*) FROM app WHERE account = :account;",
        //         params! {
        //             "account" => &self.address,
        //         },
        //     )?;
        //     result.ok_or_else(|| anyhow!("Error getting apps total"))
        Ok(0)
    }

    pub fn get_apps(&self, page: u32, size: u32) -> Result<Vec<App>> {
        // let mut conn = db::get_connection()?;
        // let offset = (page - 1) * size;
        // let apps:Vec<App> = conn.exec(
        //     "SELECT account, id, name, description, chain, network, api_key, today_requests, created_at, http_link, websocket_link FROM app WHERE account = :address LIMIT :offset, :size;",
        //     params! {
        //         "address" => &self.address,
        //         "offset" => offset,
        //         "size" => size,
        //     },
        // ).map(|row:Vec<(String,u32,String,String,String,String,String,u32,String,String,String)>| {
        //     row.map(|(account, id, name, description, chain, network, api_key, today_requests, created_at, http_link, websocket_link)|{
        //         let mut a = App {
        //             account,
        //             id,
        //             name,
        //             description,
        //             chain,
        //             network,
        //             api_key,
        //             today_requests,
        //             created_at,
        //             http_link,
        //             websocket_link,
        //             ..Default::default()
        //         };
        //         a.generate_code_example();
        //         Ok(a)
        //     })
        // }).unwrap();
        // Ok(apps)
        Err(anyhow!("Not implemented"))
    }
}

// impl FromStr for Account {
//     type Err = anyhow::Error;
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Self::get(s)
//     }
// }
