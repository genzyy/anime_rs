use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub anime: String,
    pub character: String,
    pub quote: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items<T> {
    pub quotes: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteFromCharacter {
    pub quotes: Items<Quote>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteFromTitle {
    pub quotes: Vec<Quote>,
}
