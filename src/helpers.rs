use crate::constants::{QUERIES, URL};
use crate::types::Quote;
use colored::*;
use reqwest::Client;
use reqwest::StatusCode;
use std::io;
use termion;
use textwrap;

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

pub async fn get_random_quote(search_type: &String, search_query: &String) {
    let client: Client = Client::new();
    let url = format_url(search_type, search_query);

    let response = client.get(url).send().await.unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Quote>().await {
            Ok(parsed) => {
                pretty_print_quote(parsed).ok();
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

pub async fn get_quote_from_character(search_type: &String, search_query: &String) {
    let client: Client = Client::new();
    let url = format_url(search_type, search_query);

    let response = client.get(url).send().await.unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Vec<Quote>>().await {
            Ok(parsed) => {
                for quote in parsed {
                    pretty_print_quote(quote).ok();
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
                    pretty_print_quote(quote).ok();
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

pub fn pretty_print_quote(quote: Quote) -> Result<(), io::Error> {
    let (x, _y) = termion::terminal_size().unwrap();

    // clear terminal contents and put cursor on top.
    print!("\x1B[2J\x1B[1;1H");

    if x >= 90 {
        println!(
            "{}: {}",
            "anime".italic().white(),
            quote.anime.bold().cyan()
        );
        println!(
            "{}: {}",
            "character".italic().white(),
            quote.character.italic().green()
        );
        println!(
            "{}: \"{}\"",
            "quote".italic().white(),
            textwrap::fill(quote.quote.bold().white().as_ref() as &str, 60)
        );
    } else {
        println!(
            "{}: {}",
            "anime".italic().white(),
            quote.anime.bold().cyan()
        );
        println!(
            "{}: {}",
            "character".italic().white(),
            quote.character.italic().green()
        );
        println!(
            "{}: \"{}\"",
            "quote".italic().white(),
            quote.quote.bold().white()
        );
    }
    println!();
    println!();
    Ok(())
}
