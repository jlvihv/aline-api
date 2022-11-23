use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

const ETHEREUM_HTTP: &str = "http://localhost/http_node";
const ETHEREUM_WS: &str = "ws://localhost/ws_node";

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
    pub fn get_all() -> Vec<Chain> {
        vec![
            Chain::new(ChainEnum::Ethereum),
            Chain::new(ChainEnum::Bsc),
            Chain::new(ChainEnum::Polygon),
            Chain::new(ChainEnum::Avalanche),
            Chain::new(ChainEnum::Optimism),
            Chain::new(ChainEnum::ZkSync),
            Chain::new(ChainEnum::StarkWare),
            Chain::new(ChainEnum::Near),
            Chain::new(ChainEnum::Aptos),
            Chain::new(ChainEnum::Sui),
        ]
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
            "startware" => Ok(ChainEnum::StarkWare),
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
    pub http_address: String,
    pub websocket_address: String,
    pub networks: Vec<Network>,
}

impl Chain {
    pub fn new(chain: ChainEnum) -> Self {
        match chain {
            ChainEnum::Ethereum => Self {
                name: chain.to_string(),
                http_address: ETHEREUM_HTTP.to_string(),
                websocket_address: ETHEREUM_WS.to_string(),
                networks: vec![Network::new("Mainnet"), Network::new("Testnet")],
            },
            _ => Self {
                name: chain.to_string(),
                http_address: ETHEREUM_HTTP.to_string(),
                websocket_address: ETHEREUM_WS.to_string(),
                networks: vec![Network::new("Mainnet"), Network::new("Testnet")],
            },
        }
    }
    pub fn have_network(&self, network: &str) -> bool {
        self.networks.iter().any(|n| n.name == network)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum NetworkEnum {
    Mainnet,
    Testnet,
}

#[derive(Deserialize, Serialize)]
pub struct Network {
    pub name: String,
}

impl Network {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for NetworkEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkEnum::Mainnet => write!(f, "Mainnet"),
            NetworkEnum::Testnet => write!(f, "Testnet"),
        }
    }
}

impl FromStr for NetworkEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" => Ok(NetworkEnum::Mainnet),
            "testnet" => Ok(NetworkEnum::Testnet),
            _ => Err(format!("{} is not a valid network", s)),
        }
    }
}
