fn attribute_prefix(attribute: &str, prefix: &str) -> String {
    format!("[{}^={}]", attribute, prefix)
}

fn attribute_suffix(attribute: &str, prefix: &str) -> String {
    format!("[{}$={}]", attribute, prefix)
}

// fn attribute_eq(attribute: &str, prefix: &str) -> String {
//     format!("[{}={}]", attribute, prefix)
// }

fn attribute_contains(attribute: &str, substring: &str) -> String {
    format!("[{}*={}]", attribute, substring)
}

pub fn generate_selector(
    attribute: &str, 
    element: Option<&str>,
    prefix: Option<&str>,
    suffix: Option<&str>,
    contains: Option<&str>
) -> String {
    let element_selector = element.unwrap_or("");
    let prefix_selector = prefix.map(|prefix| attribute_prefix(attribute, prefix)).unwrap_or_else(|| "".to_string());
    let suffix_selector = suffix.map(|suffix| attribute_suffix(attribute, suffix)).unwrap_or_else(|| "".to_string());
    let contains_selector = contains.map(|substring| attribute_contains(attribute, substring)).unwrap_or_else(|| "".to_string());
    format!("{}[{}]{}{}{}", element_selector, attribute, prefix_selector, suffix_selector, contains_selector)
}