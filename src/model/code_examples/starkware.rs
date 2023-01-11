use super::{examples::CodeExample, tools::generate};

const STARKWARE_JS_EXAMPLE: &str = r#"import { Provider } from "starknet";

const provider = new Provider({ rpc: { nodeUrl: "${link}" } });
let chainID = await provider.getChainId();

console.log(chainID);"#;

const STARKWARE_CLI_EXAMPLE: &str = r#"curl --url ${link} \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"starknet_blockNumber","params":[],"id":1}'"#;

const STARKWARE_PYTHON_EXAMPLE: &str = r#"import requests
r = requests.post('${link}', json={
                  'jsonrpc': '2.0', 'method': 'starknet_blockNumber', 'id': 1})
print(r.json())
"#;

const STARKWARE_GO_EXAMPLE: &str = r#"package main

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
		"method":  "starknet_blockNumber",
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

pub fn get_starkware_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(STARKWARE_JS_EXAMPLE, link),
        cli: generate(STARKWARE_CLI_EXAMPLE, link),
        python: generate(STARKWARE_PYTHON_EXAMPLE, link),
        go: generate(STARKWARE_GO_EXAMPLE, link),
    }
}
