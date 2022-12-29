use std::collections::HashMap;

use text_template::Template;

pub fn generate(template: &str, link: &str) -> String {
    let mut values = HashMap::new();
    values.insert("link", link);
    Template::from(template).fill_in(&values).to_string()
}
