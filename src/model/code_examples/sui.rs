use super::{examples::CodeExample, tools::generate};

pub const SUI_JS_EXAMPLE: &str = r#"var request = require('request');
request.post('${link}', {
    json: {
        'jsonrpc': '2.0',
        'method': 'rpc.discover',
        'id': 1
    }
}, (error, resp, body) => {
    if (!error && resp.statusCode == 200) {
        console.log(body);
    }
})"#;

pub const SUI_CLI_EXAMPLE: &str = r#"curl --url ${link} \
-X POST \
-H "Content-Type: application/json" \
-d '{"jsonrpc":"2.0","method":"rpc.discover","id":1}'
"#;

pub const SUI_PYTHON_EXAMPLE: &str = r#"import requests
r = requests.post('${link}', json={
                  'jsonrpc': '2.0', 'method': 'rpc.discover', 'id': 1})
print(r.json())
"#;

pub const SUI_GO_EXAMPLE: &str = r#"package main

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
		"method":  "rpc.discover",
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

pub fn get_sui_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(SUI_JS_EXAMPLE, link),
        cli: generate(SUI_CLI_EXAMPLE, link),
        python: generate(SUI_PYTHON_EXAMPLE, link),
        go: generate(SUI_GO_EXAMPLE, link),
    }
}
