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
    app_id_index: i32,
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
        let user = sqlx::query!("SELECT * FROM accounts WHERE address = $1", address)
            .fetch_one(&db::get_pool()?)
            .await
            .map(|a| Self {
                address: a.address,
                created_at: a.created_at,
                app_id_index: a.app_id_index,
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
        let n = sqlx::query!(
            "DELETE FROM apps WHERE account = $1 AND id = $2;",
            self.address,
            id.parse::<i32>()?
        )
        .execute(&db::get_pool()?)
        .await?;
        if 0 == n.rows_affected() {
            Err(anyhow!("App not found"))
        } else {
            Ok(())
        }
    }

    async fn save(&self) -> Result<()> {
        sqlx::query!(
            "INSERT INTO accounts (
                address, created_at, app_id_index
            ) VALUES (
                $1, $2, $3
            )
            ON CONFLICT (address)
            DO UPDATE SET app_id_index = $3;",
            self.address,
            self.created_at,
            self.app_id_index,
        )
        .execute(&db::get_pool()?)
        .await?;
        Ok(())
    }

    pub async fn get_apps_total(&self) -> Result<i64> {
        sqlx::query!(
            "SELECT COUNT(*) as total FROM apps WHERE account = $1",
            self.address
        )
        .fetch_one(&db::get_pool()?)
        .await?
        .total
        .ok_or_else(|| anyhow!("Failed to get total"))
    }

    pub async fn get_apps(&self, page: i64, size: i64) -> Result<Vec<App>> {
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
            LIMIT $2
            OFFSET $3;",
            self.address,
            size,
            offset,
        )
        .fetch_all(&db::get_pool()?)
        .await?;
        apps.into_iter()
            .map(|a| {
                let mut app = App {
                    account: a.account,
                    id: a.id,
                    name: a.name,
                    description: a.description,
                    chain: a.chain,
                    network: a.network,
                    api_key: a.api_key,
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
