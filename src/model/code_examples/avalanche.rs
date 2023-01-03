use super::{examples::CodeExample, tools::generate};

const AVALANCHE_JS_EXAMPLE: &str = "Writing...";
const AVALANCHE_CLI_EXAMPLE: &str = "Writing...";
const AVALANCHE_PYTHON_EXAMPLE: &str = "Writing...";
const AVALANCHE_GO_EXAMPLE: &str = "Writing...";

pub fn get_avalanche_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(AVALANCHE_JS_EXAMPLE, link),
        cli: generate(AVALANCHE_CLI_EXAMPLE, link),
        python: generate(AVALANCHE_PYTHON_EXAMPLE, link),
        go: generate(AVALANCHE_GO_EXAMPLE, link),
    }
}
