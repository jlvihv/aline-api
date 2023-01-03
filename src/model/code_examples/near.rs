use super::{examples::CodeExample, tools::generate};

pub const NEAR_JS_EXAMPLE: &str = r#"var request = require('request');
request.post('${link}', {
    json: {
        "jsonrpc": "2.0",
        "id": "dontcare",
        "method": "block",
        "params": { "finality": "final" }
    }
}, (error, resp, body) => {
    if (!error && resp.statusCode == 200) {
        console.log(body);
    } else {
        console.error(error);
    }
})"#;

pub const NEAR_CLI_EXAMPLE: &str = r#"curl --url ${link} \
-X POST \
-H "Content-Type: application/json" \
-d '{
  "jsonrpc": "2.0",
  "id": "dontcare",
  "method": "block",
  "params": { "finality": "final" }
}'"#;

pub const NEAR_PYTHON_EXAMPLE: &str = r#"import requests
r = requests.post('${link}', json={
    "jsonrpc": "2.0",
    "id": "dontcare",
    "method": "block",
    "params": {"finality": "final"}
})
print(r.json())
"#;

pub const NEAR_GO_EXAMPLE: &str = r#"package main

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
		"id":      "dontcare",
		"method":  "block",
		"params":  map[string]string{"finality": "final"},
	}
	bytesData, _ := json.Marshal(data)
	req, _ := http.NewRequest("POST", "${link}", bytes.NewReader(bytesData))
	req.Header.Set("Content-Type", "application/json")
	resp, _ := client.Do(req)
	body, _ := io.ReadAll(resp.Body)
	fmt.Print(string(body))
}
"#;

pub fn get_near_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(NEAR_JS_EXAMPLE, link),
        cli: generate(NEAR_CLI_EXAMPLE, link),
        python: generate(NEAR_PYTHON_EXAMPLE, link),
        go: generate(NEAR_GO_EXAMPLE, link),
    }
}
