use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use chrono::prelude::*;

use super::{
    app::{self, App},
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
        let user = sqlx::query!(
            "SELECT address, created_at, app_id_index FROM accounts WHERE address = $1",
            address
        )
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
        app::App::delete(&self.address, id.parse::<i32>()?).await
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
        app::App::get_total(&self.address).await
    }

    pub async fn get_apps(&self, page: i64, size: i64) -> Result<Vec<App>> {
        app::App::get_with_page(&self.address, page, size).await
    }
}
