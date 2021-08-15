use clap::{Arg, App};

mod attribute_scraper;

fn main() {
    let matches = App::new("URL Scraper")
        .version("0.1.0")
        .about("Scrape websites for URLs")
        .arg(Arg::with_name("url")
            .help("URL to webpage to scrape")
            .required(true))
        .arg(Arg::with_name("element")
            .short("e")
            .long("element")
            .help("Element type to search for href attributes for")
            .takes_value(true))
        .arg(Arg::with_name("prefix")
            .short("p")
            .long("prefix")
            .help("Prefix to filter href values by")
            .takes_value(true))
        .arg(Arg::with_name("suffix")
            .short("s")
            .long("suffix")
            .help("Suffix to filter href values by")
            .takes_value(true))
        .arg(Arg::with_name("contains")
            .short("c")
            .long("contains")
            .help("Substring to filter href values by")
            .takes_value(true))
        .get_matches();
    
    let url = matches.value_of("url").unwrap();
    let element = matches.value_of("element");
    let prefix = matches.value_of("prefix");
    let suffix = matches.value_of("suffix");
    let contains = matches.value_of("contains");

    attribute_scraper::print_attribute_values(url, None, "href", element, prefix, suffix, contains);
}
