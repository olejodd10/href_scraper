use curl::easy::{Easy2, Handler, WriteError};
use std::path::Path;

// https://docs.rs/curl/0.4.38/curl/easy/trait.Handler.html
struct Collector(Vec<u8>);

impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

pub fn print_href_values(url: &str, cookie_file_path: Option<&Path>) {
    let mut hrefs = scraped_href_values(url, cookie_file_path).unwrap();
    // hrefs.sort_by(|s1,s2| s1.len().cmp(&s2.len()));
    hrefs.sort();
    if hrefs.is_empty() {
        println!("No hrefs found for {}", url);
    } else {
        for href in hrefs {
            println!("{}", href);
        }
    }
}

fn scraped_href_values(url: &str, cookie_file_path: Option<&Path>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.url(url)?;

    if let Some(cookie_file_path) = cookie_file_path {
        easy.cookie_file(cookie_file_path).unwrap();
    }

    easy.fail_on_error(true)?; //Viktig for å faile på 401
    
    easy.perform()?;

    let content = String::from_utf8(easy.get_ref().0.clone()).expect("Error converting content to String");
    
    let document = scraper::Html::parse_document(&content);
    let href_selector = scraper::Selector::parse("[href]").unwrap(); // Everything that has a href attribute

    let href_values = document.select(&href_selector).map(|element| element.value().attr("href").unwrap().to_string()).collect();

    Ok(href_values)
}