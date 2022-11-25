use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use chrono::prelude::*;

use super::{
    app::App,
    chain::{ChainEnum, NetworkEnum},
    db,
};

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Account {
    pub address: String,
    pub created_at: String,
    app_id_index: u32,
}

impl Account {
    pub async fn new(address: &str) -> Result<Self> {
        let a = Self {
            address: address.to_string(),
            created_at: Local::now().to_string(),
            app_id_index: 0,
        };
        a.save().await?;
        Ok(a)
    }

    pub async fn get(address: &str) -> Result<Self> {
        let pool = db::get_db_pool();
        let user = sqlx::query!("SELECT * FROM account WHERE address = $1", address)
            .fetch_one(&pool)
            .await
            .map(|a| Self {
                address: a.address,
                created_at: a.created_at,
                app_id_index: a.app_id_index as u32,
            })
            .map_err(|e| anyhow!(e));
        if user.is_ok() {
            user
        } else {
            Self::new(address).await
        }
    }

    pub async fn create_app(
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
        )
        .await?;
        self.app_id_index += 1;
        self.save().await?;
        Ok(app)
    }

    pub async fn delete_app(&self, id: &str) -> Result<()> {
        let pool = db::get_db_pool();
        let n = sqlx::query!(
            "DELETE FROM app WHERE account = $1 AND id = $2;",
            self.address,
            id.parse::<i32>()?
        )
        .execute(&pool)
        .await?;
        if 0 == n.rows_affected() {
            Err(anyhow!("App not found"))
        } else {
            Ok(())
        }
    }

    async fn save(&self) -> Result<()> {
        let pool = db::get_db_pool();
        sqlx::query!(
            "INSERT INTO account (
                address, created_at, app_id_index
            ) VALUES (
                $1, $2, $3
            )
            ON CONFLICT (address)
            DO UPDATE SET app_id_index = $3;",
            self.address,
            self.created_at,
            self.app_id_index as i32,
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn get_apps_total(&self) -> Result<u32> {
        let pool = db::get_db_pool();

        let row = sqlx::query!(
            "SELECT COUNT(*) as total FROM app WHERE account = $1",
            self.address
        )
        .fetch_one(&pool)
        .await?;

        row.total
            .map(|t| t as u32)
            .ok_or_else(|| anyhow!("Failed to get total"))
    }

    pub async fn get_apps(&self, page: u32, size: u32) -> Result<Vec<App>> {
        if page == 0 {
            return Err(anyhow!("Page must be greater than 0"));
        }
        let offset = (page - 1) * size;
        let pool = db::get_db_pool();
        let apps = sqlx::query!(
            "SELECT
                account, id, name, description, chain, network, api_key, today_requests, created_at, http_link, websocket_link
            FROM app
            WHERE
                account = $1
            LIMIT $2
            OFFSET $3;",
            self.address,
            size as i64,
            offset as i64,
        )
        .fetch_all(&pool).await?;
        apps.into_iter()
            .map(|a| {
                let mut app = App {
                    account: a.account,
                    id: a.id as u32,
                    name: a.name,
                    description: a.description,
                    chain: a.chain,
                    network: a.network,
                    api_key: a.api_key,
                    today_requests: a.today_requests as u32,
                    created_at: a.created_at,
                    http_link: a.http_link,
                    websocket_link: a.websocket_link,
                    ..Default::default()
                };
                app.generate_code_example();
                Ok(app)
            })
            .collect()
    }
}
