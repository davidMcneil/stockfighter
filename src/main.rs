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
const SYMBOL: &'static str = "FOOBAR";
#[allow(dead_code)]
const ACCOUNT: &'static str = "EXB123456";

// // Authentification for actual level server
// #[allow(dead_code)]
// const VENUE: &'static str = "IRHEX";
// #[allow(dead_code)]
// const SYMBOL: &'static str = "IZFI";
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

    let order_result = order_stock(ACCOUNT, VENUE, SYMBOL, 2, 5, Sell, Limit);
    // println!("{:?}", order_result);
    println!("{:?}", stock_orderbook(VENUE, SYMBOL));
    println!("{:?}", stock_quote(VENUE, SYMBOL));
    println!("{:?}", order_status(VENUE, SYMBOL, order_result.id));
    println!("{:?}", cancel_order(VENUE, SYMBOL, order_result.id));
    println!("{:?}", stock_orderbook(VENUE, SYMBOL));

    println!("{:?}", venue_order_statuses(VENUE, ACCOUNT));
    println!("{:?}", stock_order_statuses(VENUE, ACCOUNT, SYMBOL));

    // std::thread::sleep(std::time::Duration::from_millis(500));
}
