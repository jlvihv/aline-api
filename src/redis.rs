use anyhow::{Ok, Result};
pub struct Redis {
    pub conn: redis::Connection,
}

impl Redis {
    pub fn new(address: &str) -> Result<Self> {
        let client = redis::Client::open(address)?;
        let mut conn = client.get_connection()?;
        Ok(Self { conn })
    }
}
