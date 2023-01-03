use super::{examples::CodeExample, tools::generate};

const APTOS_JS_EXAMPLE: &str = "Writing...";
const APTOS_CLI_EXAMPLE: &str = "Writing...";
const APTOS_PYTHON_EXAMPLE: &str = "Writing...";
const APTOS_GO_EXAMPLE: &str = "Writing...";

pub fn get_aptos_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(APTOS_JS_EXAMPLE, link),
        cli: generate(APTOS_CLI_EXAMPLE, link),
        python: generate(APTOS_PYTHON_EXAMPLE, link),
        go: generate(APTOS_GO_EXAMPLE, link),
    }
}
