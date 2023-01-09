use super::{
    ethereum::{ETH_CLI_EXAMPLE, ETH_GO_EXAMPLE, ETH_JS_EXAMPLE, ETH_PYTHON_EXAMPLE},
    examples::CodeExample,
    tools::generate,
};
pub fn get_bsc_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(ETH_JS_EXAMPLE, link),
        cli: generate(ETH_CLI_EXAMPLE, link),
        python: generate(ETH_PYTHON_EXAMPLE, link),
        go: generate(ETH_GO_EXAMPLE, link),
    }
}
