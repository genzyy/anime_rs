use crate::constants::{QUERIES, URL};
use crate::types::Quote;
use reqwest::Client;
use reqwest::StatusCode;

pub fn format_url(query: &String, parameter: &String) -> String {
    let mut url = String::new();
    if query == QUERIES[0] {
        url = format!("{URL}quotes/character?name={name}", name = parameter);
    } else if query == QUERIES[1] {
        url = format!("{URL}quotes/anime?title={title}", title = parameter);
    } else if query == QUERIES[2] {
        url = format!("{URL}random")
    }
    return url;
}

pub fn print_quote(quote: Quote) {
    println!("{}", quote.anime);
    println!("{}", quote.character);
    println!("{}", quote.quote);
    println!();
    println!();
}

pub async fn get_random_quote(search_type: &String, search_query: &String) {
    let client: Client = Client::new();
    let url = format_url(search_type, search_query);

    let response = client.get(url).send().await.unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Quote>().await {
            Ok(parsed) => print_quote(parsed),
            Err(_) => {
                println!("something wrong with the data");
            }
        },
        StatusCode::UNAUTHORIZED => {
            println!("authentication problem");
        }
        _other => {
            println!("something went wrong.");
        }
    }
}

pub async fn get_quote_from_character(search_type: &String, search_query: &String) {
    let client: Client = Client::new();
    let url = format_url(search_type, search_query);

    let response = client.get(url).send().await.unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Vec<Quote>>().await {
            Ok(parsed) => {
                for quote in parsed {
                    print_quote(quote);
                }
            }
            Err(e) => {
                println!("something wrong with the data");
                println!("{}", e);
            }
        },
        StatusCode::UNAUTHORIZED => {
            println!("authentication problem");
        }
        _other => {
            println!("something went wrong.");
        }
    }
}

pub async fn get_quote_from_title(search_type: &String, search_query: &String) {
    let client: Client = Client::new();
    let url = format_url(search_type, search_query);

    let response = client.get(url).send().await.unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Vec<Quote>>().await {
            Ok(parsed) => {
                for quote in parsed {
                    print_quote(quote);
                }
            }
            Err(_) => {
                println!("something wrong with the data");
            }
        },
        StatusCode::UNAUTHORIZED => {
            println!("authentication problem");
        }
        _other => {
            println!("something went wrong.");
        }
    }
}
