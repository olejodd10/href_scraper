use clap::{Arg, App, SubCommand};
use std::path::Path;

mod download;
mod href_scraper;

fn main() {
    let matches = App::new("URL Scraper")
        .version("0.1.0")
        .about("Scrape websites for URLs")
        .subcommand(SubCommand::with_name("download")
            .about("Download response")
            .arg(Arg::with_name("path")
                .help("Path for downloaded file")
                .required(true)) // TODO: default corresponding to filename
            .arg(Arg::with_name("unzip")
                .short("u")
                .long("unzip")
                .help("Attempt to unzip downloaded file"))
            .arg(Arg::with_name("overwrite")
                .short("o")
                .long("overwrite")
                .help("Overwrite if file exists")))
        .arg(Arg::with_name("url")
            .help("URL to download")
            .required(true))
        .get_matches();
    
    let url = matches.value_of("url").unwrap();

    if let Some(sub_download) = matches.subcommand_matches("download") {
        let path = Path::new(sub_download.value_of("path").unwrap());
        let overwrite = sub_download.is_present("overwrite");
        if sub_download.is_present("unzip") {
            download::download_and_unzip(url, path, None, overwrite).unwrap();
        } else {
            download::download_file(url, path, None, overwrite).unwrap();
        }
    } else {
        href_scraper::print_href_values(url, None);
    }

}
