use super::{examples::CodeExample, tools::generate};

const STARKWARE_JS_EXAMPLE: &str = "Writing...";
const STARKWARE_CLI_EXAMPLE: &str = "Writing...";
const STARKWARE_PYTHON_EXAMPLE: &str = "Writing...";
const STARKWARE_GO_EXAMPLE: &str = "Writing...";

pub fn get_starkware_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(STARKWARE_JS_EXAMPLE, link),
        cli: generate(STARKWARE_CLI_EXAMPLE, link),
        python: generate(STARKWARE_PYTHON_EXAMPLE, link),
        go: generate(STARKWARE_GO_EXAMPLE, link),
    }
}
