#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

mod types;
mod api;

// use types::*;
use types::Direction::*;
use types::OrderType::*;
use api::*;

// Authentification for test server
#[allow(dead_code)]
const VENUE: &'static str = "TESTEX";
#[allow(dead_code)]
const STOCK: &'static str = "FOOBAR";
#[allow(dead_code)]
const ACCOUNT: &'static str = "EXB123456";

// // Authentification for actual level server
// #[allow(dead_code)]
// const VENUE: &'static str = "IRHEX";
// #[allow(dead_code)]
// const STOCK: &'static str = "IZFI";
// #[allow(dead_code)]
// const ACCOUNT: &'static str = "ELB34783810";

fn main() {
    println!("---StockFighter---\n");
    if !heartbeat() {
        panic!("Could not connect to StockFighter API.");
    }
    if !venue_heartbeat(VENUE) {
        panic!("Could not connect to the venue {}.", VENUE);
    }
    let stocks = venue_stocks(VENUE);
    println!("On venue {} the following stocks are available for trade:", VENUE);
    for stock in stocks.iter() {
        println!("    {} -> {}", stock.symbol, stock.name);
    }

    let order_result = order_stock(ACCOUNT, VENUE, STOCK, 2, 5, Sell, Limit);
    // println!("{:?}", order_result);
    // println!("{:?}", stock_orderbook(VENUE, STOCK));
    // println!("{:?}", stock_quote(VENUE, STOCK));
    println!("{:?}", order_status(VENUE, STOCK, order_result.id));

    // std::thread::sleep(std::time::Duration::from_millis(500));
}
