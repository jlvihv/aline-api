use super::{ethereum::*, examples::CodeExample, tools::generate};

pub fn get_polygon_examples(link: &str) -> CodeExample {
    CodeExample {
        js: generate(ETH_JS_EXAMPLE, link),
        cli: generate(ETH_CLI_EXAMPLE, link),
        python: generate(ETH_PYTHON_EXAMPLE, link),
        go: generate(ETH_GO_EXAMPLE, link),
    }
}


