use scraper::{Html, Selector};
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::collections::HashSet;

fn extract_hrefs(html: &str) -> Vec<String> {
    let fragment = Html::parse_fragment(html);
    let card_selector = Selector::parse(".card").unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let mut hrefs = Vec::new();

    for card in fragment.select(&card_selector) {
        for link in card.select(&link_selector) {
            if let Some(href) = link.value().attr("href") {
                hrefs.push(href.to_string());
            }
        }
    }

    hrefs
}

fn read_file_contents(filename: &str) -> Result<String, Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        return Err(Error::new(ErrorKind::Other, e.to_string()));
    }

    Ok(contents)
}

fn remove_duplicates(urls: Vec<String>) -> Vec<String> {
    let mut set = HashSet::new();
    let mut result = Vec::new();

    for url in urls {
        if set.insert(url.clone()) {
            // if the URL is not already in the set, add it to the result vector
            result.push(url);
        }
    }

    result
}

fn write_url_file(list: Vec<String>) {
    let content = list.join("\n");
    fs::write("data\\urls.txt", content).expect("Unable to write file"); // Write to file
}

// Pass in file path to workshop collection html file
pub fn build_url_list(collpath: String ) {
    match read_file_contents(&collpath) {
        Ok(contents) => {
            let modlinks = extract_hrefs(&contents);
            let uniquelist = remove_duplicates(modlinks);

            // Write links to urls.txt file
            write_url_file(uniquelist);
        },
        Err(e) => {
            panic!("Failed to read Collection html source file: {}", e);
        }
    };

}

pub fn combine_lists(file1: String, file2: String) {
    // Open the first file and read its contents
    let file1 = File::open(file1).expect("Failed to open file 1");
    let reader1 = BufReader::new(file1);
    let urls1: Vec<String> = reader1.lines().map(|line| line.unwrap()).collect();

    // Open the second file and read its contents
    let file2 = File::open(file2).expect("Failed to open file 2");
    let reader2 = BufReader::new(file2);
    let urls2: Vec<String> = reader2.lines().map(|line| line.unwrap()).collect();

    // Combine the lists and remove duplicates
    let mut all_urls = HashSet::new();
    all_urls.extend(urls1);
    all_urls.extend(urls2);

    // Convert the HashSet to a Vec<String> and print the combined list of URLs
    let urls: Vec<String> = all_urls.into_iter().collect();
    
    write_url_file(urls);
}