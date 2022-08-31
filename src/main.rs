use constants::QUERIES;
use std::env;

mod constants;
mod helpers;
mod types;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut search_type: &String = &String::from(QUERIES[2]);
    let mut search_query: &String = &String::from("naruto");

    if args.len() == 1 {
        helpers::get_random_quote(search_type, search_query).await;
    } else if args.len() >= 3 {
        search_type = &args[1];
        let temp = args[2..].join(" ");
        search_query = &temp;
        if search_type == QUERIES[0] {
            helpers::get_quote_from_character(search_type, search_query).await;
        } else if search_type == QUERIES[1] {
            helpers::get_quote_from_title(search_type, search_query).await;
        }
    } else {
        println!("atleast 2 arguments are required for custom search");
    }
}
