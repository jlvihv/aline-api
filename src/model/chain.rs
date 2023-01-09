use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

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
    pub http_address: String,
    pub websocket_address: String,
    pub networks: Vec<NetworkEnum>,
    pub is_available: bool,
}

impl Chain {
    pub fn new(chain: ChainEnum) -> Self {
        match chain {
            ChainEnum::Ethereum => Self {
                name: chain.to_string(),
                http_address: get_chain_link(&chain).0,
                websocket_address: get_chain_link(&chain).1,
                networks: vec![NetworkEnum::Mainnet, NetworkEnum::Testnet(Testnet::Goerli)],
                is_available: get_chain_link(&chain).0 != "Not supported yet"
                    || get_chain_link(&chain).1 != "Not supported yet",
            },
            _ => Self {
                name: chain.to_string(),
                http_address: get_chain_link(&chain).0,
                websocket_address: get_chain_link(&chain).1,
                networks: vec![NetworkEnum::Mainnet],
                is_available: get_chain_link(&chain).0 != "Not supported yet"
                    || get_chain_link(&chain).1 != "Not supported yet",
            },
        }
    }
    pub fn have_network(&self, network: &str) -> bool {
        let network = match network.parse::<NetworkEnum>() {
            Ok(n) => n,
            Err(_) => return false,
        };
        self.networks.iter().any(|n| n == &network)
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
            NetworkEnum::Testnet(Testnet::Ropsten) => write!(f, "Testnet - Ropsten"),
            NetworkEnum::Testnet(Testnet::Rinkeby) => write!(f, "Testnet - Rinkeby"),
            NetworkEnum::Testnet(Testnet::Kovan) => write!(f, "Testnet - Kovan"),
            NetworkEnum::Testnet(Testnet::Goerli) => write!(f, "Testnet - Goerli"),
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

/// 返回链的 http 和 websocket 地址
fn get_chain_link(chain: &ChainEnum) -> (String, String) {
    let chain_http = format!("{}_HTTP", chain.to_string().to_uppercase());
    let chain_ws = format!("{}_WS", chain.to_string().to_uppercase());
    let http_link = std::env::var(chain_http).unwrap_or_else(|_| "Not supported yet".to_string());
    let ws_link = std::env::var(chain_ws).unwrap_or_else(|_| "Not supported yet".to_string());
    (http_link, ws_link)
}
