use std::{cell::RefCell, sync::Mutex};

use super::log::Log;
use anyhow::Result;
use chrono::{DateTime, Utc};
use once_cell::sync::OnceCell;

static CACHE: OnceCell<Mutex<RefCell<LogCache>>> = OnceCell::new();

pub async fn init() -> std::result::Result<(), Mutex<RefCell<LogCache>>> {
    let cache = match LogCache::new() {
        Ok(cache) => cache,
        Err(e) => {
            tracing::error!("cache log failed: {}", e);
            LogCache::default()
        }
    };
    CACHE.set(Mutex::new(RefCell::new(cache)))
}

#[derive(Debug, Clone)]
pub struct LogCache {
    pub data: Vec<Log>,
    pub latest_time: DateTime<Utc>,
}

impl LogCache {
    pub fn new() -> Result<Self> {
        let logs = Log::parse_file(&Self::get_log_path()?)?;
        Ok(Self {
            data: logs,
            latest_time: Utc::now(),
        })
    }

    fn get_logs() -> Result<Vec<Log>> {
        let logs = Log::parse_file(&Self::get_log_path()?)?;
        Ok(logs)
    }

    fn get_log_path() -> Result<String> {
        std::env::var("PARSE_LOG_FILE").map_err(|_| anyhow::anyhow!("PARSE_LOG_FILE must be set"))
    }

    pub async fn get() -> Result<Self> {
        match CACHE.get() {
            Some(cache) => {
                let cache = match cache.lock() {
                    Ok(cache) => cache,
                    Err(e) => {
                        tracing::error!("cache log failed: {}", e);
                        return Err(anyhow::anyhow!("cache log failed: {}", e));
                    }
                };
                let mut cache = cache.borrow_mut();
                if Utc::now()
                    .signed_duration_since(cache.latest_time)
                    .num_seconds()
                    > 1
                {
                    // tracing::info!("cache expired, update cache");
                    cache.data = Self::get_logs()?;
                    cache.latest_time = Utc::now();
                }
                Ok(cache.clone())
            }
            None => Err(anyhow::anyhow!("Log cache not initialized")),
        }
    }
}

impl Default for LogCache {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            latest_time: Utc::now(),
        }
    }
}
