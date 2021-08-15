use curl::easy::{Easy2, Handler, WriteError};
use std::path::Path;

mod selector_utils;
use selector_utils::generate_selector;

// https://docs.rs/curl/0.4.38/curl/easy/trait.Handler.html
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn print_attribute_values(
    url: &str, 
    cookie_file_path: Option<&Path>, 
    attribute: &str, 
    element: Option<&str>,
    prefix: Option<&str>,
    suffix: Option<&str>,
    contains: Option<&str>
) {
    let mut values = collect_attribute_values(url, cookie_file_path, attribute, element, prefix, suffix, contains).unwrap();
    // values.sort_by(|s1,s2| s1.len().cmp(&s2.len()));
    values.sort();
    if values.is_empty() {
        println!("No matching {} found for {}", attribute, url);
    } else {
        for value in values {
            println!("{}", value);
        }
    }
}

fn collect_attribute_values(
    url: &str, 
    cookie_file_path: Option<&Path>, 
    attribute: &str, 
    element: Option<&str>,
    prefix: Option<&str>,
    suffix: Option<&str>,
    contains: Option<&str>
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.url(url)?;

    if let Some(cookie_file_path) = cookie_file_path {
        easy.cookie_file(cookie_file_path).unwrap();
    }

    easy.fail_on_error(true)?; //Viktig for å faile på 401
    
    easy.perform()?;

    let content = String::from_utf8(easy.get_ref().0.clone()).expect("Error converting content to String");
    
    let document = scraper::Html::parse_document(&content);
    let attribute_selector = scraper::Selector::parse(&generate_selector(attribute, element, prefix, suffix, contains)).unwrap();

    let attribute_values = document.select(&attribute_selector).map(|element| element.value().attr(attribute).unwrap().to_string()).collect();

    Ok(attribute_values)
}