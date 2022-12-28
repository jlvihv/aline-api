use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use text_template::*;

use super::chain::ChainEnum;

const ETH_JS_EXAMPLE: &str = r#"var request = require('request');
request.post('${link}', {
    json: {
        'jsonrpc': '2.0',
        'method': 'eth_blockNumber',
        'params': [],
        'id': 1
    }
}, (error, resp, body) => {
    if (!error && resp.statusCode == 200) {
        console.log(body);
    }
})"#;

const ETH_CLI_EXAMPLE: &str = r#"curl --url ${link} \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'"#;

const ETH_PYTHON_EXAMPLE: &str = r#"import requests
r = requests.post('${link}', json={
                  'jsonrpc': '2.0', 'method': 'eth_blockNumber', 'params': [], 'id': 1})
print(r.json())
"#;

const ETH_GO_EXAMPLE: &str = r#"package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

func main() {
	client := &http.Client{}
	data := map[string]any{
		"jsonrpc": "2.0",
		"method":  "eth_blockNumber",
		"params":  []string{},
		"id":      1,
	}
	bytesData, _ := json.Marshal(data)
	req, _ := http.NewRequest("POST", "${link}", bytes.NewReader(bytesData))
	req.Header.Set("Content-Type", "application/json")
	resp, _ := client.Do(req)
	body, _ := io.ReadAll(resp.Body)
	fmt.Print(string(body))
}
"#;

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
        ChainEnum::Ethereum => CodeExample {
            js: generate(ETH_JS_EXAMPLE, link),
            cli: generate(ETH_CLI_EXAMPLE, link),
            python: generate(ETH_PYTHON_EXAMPLE, link),
            go: generate(ETH_GO_EXAMPLE, link),
        },
        _ => CodeExample {
            js: generate(ETH_JS_EXAMPLE, link),
            cli: generate(ETH_CLI_EXAMPLE, link),
            python: generate(ETH_PYTHON_EXAMPLE, link),
            go: generate(ETH_GO_EXAMPLE, link),
        },
    }
}

fn generate(template: &str, link: &str) -> String {
    let mut values = HashMap::new();
    values.insert("link", link);
    Template::from(template).fill_in(&values).to_string()
}
