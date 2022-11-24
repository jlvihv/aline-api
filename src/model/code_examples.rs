use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str::FromStr};
use text_template::*;

const JS_EXAMPLE: &str = r#"package main

import "fmt"

func main() {
	fmt.Println("Hello ${link}!")
}"#;

const CLI_EXAMPLE: &str = r#"package main

import "fmt"

func main() {
	fmt.Println("Hello ${link}!")
}"#;

const PYTHON_EXAMPLE: &str = r#"package main

import "fmt"

func main() {
	fmt.Println("Hello ${link}!")
}"#;

const GO_EXAMPLE: &str = r#"package main

import "fmt"

func main() {
	fmt.Println("Hello ${link}!")
}"#;

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

pub fn get_code_example(link: &str) -> CodeExample {
    CodeExample {
        js: generate(JS_EXAMPLE, link),
        cli: generate(CLI_EXAMPLE, link),
        python: generate(PYTHON_EXAMPLE, link),
        go: generate(GO_EXAMPLE, link),
    }
}

fn generate(template: &str, link: &str) -> String {
    let mut values = HashMap::new();
    values.insert("link", link);
    Template::from(template).fill_in(&values).to_string()
}
