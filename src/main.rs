#[macro_use]
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;

mod types;
mod api;
mod strategy;

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
    assert!(heartbeat(), "Could not connect to StockFighter API.");
    assert!(venue_heartbeat(VENUE), "Could not connect to the venue {}.", VENUE);

    strategy::market_maker::run();
}
