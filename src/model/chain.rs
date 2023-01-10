use anyhow::anyhow;
use anyhow::Result;
use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

use super::db;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ChainEnum {
    Ethereum,
    Bsc,
    Polygon,
    Avalanche,
    Optimism,
    ZkSync,
    StarkWare,
    Near,
    Aptos,
    Sui,
}

impl ChainEnum {
    pub async fn get_all() -> Result<Vec<Chain>> {
        let mut result: Vec<Chain> = vec![];
        result.push(Chain::get(ChainEnum::Ethereum).await?);
        result.push(Chain::get(ChainEnum::Bsc).await?);
        result.push(Chain::get(ChainEnum::Polygon).await?);
        result.push(Chain::get(ChainEnum::Avalanche).await?);
        result.push(Chain::get(ChainEnum::Optimism).await?);
        result.push(Chain::get(ChainEnum::StarkWare).await?);
        result.push(Chain::get(ChainEnum::Near).await?);
        result.push(Chain::get(ChainEnum::Aptos).await?);
        result.push(Chain::get(ChainEnum::Sui).await?);
        Ok(result)
    }
}

impl fmt::Display for ChainEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainEnum::Ethereum => write!(f, "Ethereum"),
            ChainEnum::Bsc => write!(f, "Bsc"),
            ChainEnum::Polygon => write!(f, "Polygon"),
            ChainEnum::Avalanche => write!(f, "Avalanche"),
            ChainEnum::Optimism => write!(f, "Optimism"),
            ChainEnum::ZkSync => write!(f, "ZkSync"),
            ChainEnum::StarkWare => write!(f, "StarkWare"),
            ChainEnum::Near => write!(f, "Near"),
            ChainEnum::Aptos => write!(f, "Aptos"),
            ChainEnum::Sui => write!(f, "Sui"),
        }
    }
}

impl FromStr for ChainEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" => Ok(ChainEnum::Ethereum),
            "bsc" => Ok(ChainEnum::Bsc),
            "polygon" => Ok(ChainEnum::Polygon),
            "avalanche" => Ok(ChainEnum::Avalanche),
            "optimism" => Ok(ChainEnum::Optimism),
            "zksync" => Ok(ChainEnum::ZkSync),
            "starkware" => Ok(ChainEnum::StarkWare),
            "near" => Ok(ChainEnum::Near),
            "aptos" => Ok(ChainEnum::Aptos),
            "sui" => Ok(ChainEnum::Sui),
            _ => Err(format!("{} is not a valid chain", s)),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Chain {
    pub name: String,
    pub networks: Vec<Network>,
}

impl Chain {
    pub async fn get(chain: ChainEnum) -> Result<Self> {
        let network_enums = Self::get_network_enums(&chain).await?;
        let mut networks: Vec<Network> = vec![];
        for network in &network_enums {
            let network = Network::get(&chain, network).await?;
            networks.push(network);
        }
        Ok(Self {
            name: chain.to_string(),
            networks,
        })
    }

    pub fn have_network(&self, network: &str) -> bool {
        let network = match network.parse::<NetworkEnum>() {
            Ok(n) => n,
            Err(_) => return false,
        };
        self.networks.iter().any(|n| n.name == network.to_string())
    }

    pub async fn get_network(&self, network: &str) -> Result<Network> {
        let network = match network.parse::<NetworkEnum>() {
            Ok(n) => n,
            Err(_) => return Err(anyhow!("{} is not a valid network", network)),
        };
        let network = self
            .networks
            .iter()
            .find(|n| n.name == network.to_string())
            .ok_or_else(|| anyhow!("{} is not a valid network", network))?;
        Ok(network.clone())
    }

    pub async fn get_network_enums(chain: &ChainEnum) -> Result<Vec<NetworkEnum>> {
        let rows = sqlx::query!(
            r#"
        SELECT network
        FROM chains
        WHERE name = $1
        "#,
            chain.to_string().to_lowercase()
        )
        .fetch_all(&db::get_pool()?)
        .await
        .map_err(|e| {
            tracing::error!("Error getting network enums for chain => {} : {}", chain, e);
            anyhow!(
                "Failed to get chain networks from db for chain => {} : {}",
                chain,
                e
            )
        })?;
        let mut networks = vec![];
        for row in rows {
            let network = match row.network.parse::<NetworkEnum>() {
                Ok(n) => n,
                Err(_) => continue,
            };
            networks.push(network);
        }
        Ok(networks)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum NetworkEnum {
    Mainnet,
    Testnet(Testnet),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Testnet {
    Ropsten,
    Rinkeby,
    Kovan,
    Goerli,
}

impl fmt::Display for NetworkEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkEnum::Mainnet => write!(f, "Mainnet"),
            NetworkEnum::Testnet(Testnet::Ropsten) => write!(f, "Testnet-Ropsten"),
            NetworkEnum::Testnet(Testnet::Rinkeby) => write!(f, "Testnet-Rinkeby"),
            NetworkEnum::Testnet(Testnet::Kovan) => write!(f, "Testnet-Kovan"),
            NetworkEnum::Testnet(Testnet::Goerli) => write!(f, "Testnet-Goerli"),
        }
    }
}

impl FromStr for NetworkEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .as_str()
        {
            "mainnet" => Ok(NetworkEnum::Mainnet),
            "testnet-ropsten" => Ok(NetworkEnum::Testnet(Testnet::Ropsten)),
            "testnet-rinkeby" => Ok(NetworkEnum::Testnet(Testnet::Rinkeby)),
            "testnet-kovan" => Ok(NetworkEnum::Testnet(Testnet::Kovan)),
            "testnet-goerli" => Ok(NetworkEnum::Testnet(Testnet::Goerli)),
            _ => Err(format!("{} is not a valid network", s)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Network {
    pub name: String,
    pub http_address: String,
    pub websocket_address: String,
}

impl Network {
    pub async fn get(chain: &ChainEnum, network: &NetworkEnum) -> Result<Self> {
        let (http_address, websocket_address) = get_chain_link_from_db(chain, network).await?;
        Ok(Self {
            name: network.to_string(),
            http_address,
            websocket_address,
        })
    }
}

async fn get_chain_link_from_db(
    chain: &ChainEnum,
    network: &NetworkEnum,
) -> Result<(String, String)> {
    let row = sqlx::query!(
        r#"
        SELECT http_address, websocket_address
        FROM chains
        WHERE name = $1 AND network = $2
        "#,
        chain.to_string().to_lowercase(),
        network.to_string().to_lowercase()
    )
    .fetch_optional(&db::get_pool()?)
    .await
    .map_err(|e| anyhow!("Failed to get chain link from db: {}", e))?;
    let row = match row {
        Some(r) => r,
        None => {
            return Err(anyhow!(
                "No chain link found for chain => {} and network => {}",
                chain,
                network
            ))
        }
    };
    let http_address = row.http_address;
    let ws_address = row.websocket_address;

    Ok((http_address, ws_address))
}
