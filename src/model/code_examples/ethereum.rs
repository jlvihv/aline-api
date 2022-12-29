use super::{examples::CodeExample, tools::generate};

pub const ETH_JS_EXAMPLE: &str = r#"var request = require('request');
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

pub const ETH_CLI_EXAMPLE: &str = r#"curl --url ${link} \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'"#;

pub const ETH_PYTHON_EXAMPLE: &str = r#"from web3 import Web3

chain_url = "${link}"
w3 = Web3(Web3.HTTPProvider(chain_url))

# Print if web3 is successfully connected
print(w3.isConnected())

# Get the latest block number
latest_block = w3.eth.block_number
print(latest_block)

"#;

pub const ETH_GO_EXAMPLE: &str = r#"package main

import (
	"context"
	"fmt"
	"log"

	"github.com/ethereum/go-ethereum/ethclient"
)

func main() {
	client, err := ethclient.Dial("${link}")
	if err != nil {
		log.Fatal(err)
	}

	// Get the latest known block
	block, err := client.BlockByNumber(context.Background(), nil)
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("Latest block:", block.Number().Uint64())
}

"#;

pub fn get_ethereum_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(ETH_JS_EXAMPLE, link),
        cli: generate(ETH_CLI_EXAMPLE, link),
        python: generate(ETH_PYTHON_EXAMPLE, link),
        go: generate(ETH_GO_EXAMPLE, link),
    }
}
