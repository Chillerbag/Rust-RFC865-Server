// functions for dealing with db, called from adminCommands
// also called by serverHandling in the case of serveQuote.
use rusqlite::{params, Connection, Result};
use std::net::TcpStream;

struct Quote {
    quote: String,
    author: String
}

pub fn serve_quote(stream: &mut TcpStream) {
    // gives us a quote. If no quotes avaliable, AND sqllite is empty,
    // serve the default quote.
    // otherwise, wrap around, get latest date quote, find offset from that date
    // and wrap around array. 

    // TODO: dont unwrap! fix this.
    let default_quote = Quote {
        quote: dotenv::var("DEFAULT_QOTD").unwrap(),
        author: dotenv::var("DEFAULT_AUTH").unwrap()
    };

}