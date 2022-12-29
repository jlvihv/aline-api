use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::super::chain::ChainEnum;
use super::ethereum::*;
use super::sui::*;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct CodeExample {
    pub js: String,
    pub cli: String,
    pub python: String,
    pub go: String,
}

pub enum Language {
    JavaScript,
    CLI,
    Python,
    Go,
}

impl FromStr for Language {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "javascript" => Ok(Language::JavaScript),
            "cli" => Ok(Language::CLI),
            "python" => Ok(Language::Python),
            "go" => Ok(Language::Go),
            _ => Err(format!("{} is not a valid language", s)),
        }
    }
}

pub fn get_code_example(link: &str, chain_type: ChainEnum) -> CodeExample {
    match chain_type {
        ChainEnum::Ethereum => get_ethereum_examples(link),
        ChainEnum::Sui => get_sui_examples(link),
        ChainEnum::Avalanche => get_ethereum_examples(link),
        ChainEnum::Optimism => get_ethereum_examples(link),
        _ => get_ethereum_examples(link),
    }
}
