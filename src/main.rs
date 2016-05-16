#[macro_use]
extern crate hyper;
extern crate rustc_serialize;

use rustc_serialize::json::Json;
use std::io::Read;
use hyper::Client;
use hyper::header::Headers;

header!{(XStarfighterAuthorization, "X-Starfighter-Authorization") => [String]}

const API_KEY: &'static str = "3f3c319d38a5b39acdb954c14547663f77c53cbc";
const BASE_URL: &'static str = "https://api.stockfighter.io/ob/api";
const VENUE: &'static str = "TESTEX";
#[allow(dead_code)]
const STOCK: &'static str = "FOOBAR";
#[allow(dead_code)]
const ACCOUNT: &'static str = "HB61251714";

#[derive(Debug)]
struct Stock {
    name: String,
    symbol: String
}

impl Stock {
    pub fn new(name: String, symbol: String) -> Stock {
        Stock{ name: name, symbol: symbol }
    }
}

#[derive(Debug)]
struct Order {
    price: u64,
    qty: u64
}

impl Order {
    pub fn new(price: u64, qty: u64) -> Order {
        Order{ price: price, qty: qty }
    }
}

#[derive(Debug)]
struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>
}

impl OrderBook {
    pub fn new(bids: Vec<Order>, asks: Vec<Order>) -> OrderBook {
        OrderBook{ bids: bids, asks: asks }
    }
}

#[allow(dead_code)]
fn get_headers() -> Headers {
    let mut headers = Headers::new();
    headers.set(XStarfighterAuthorization(API_KEY.to_owned()));
    headers
}

fn heartbeat(client: &Client) -> bool {
    let url = format!("{}/heartbeat", BASE_URL);
    let response = client.get(&url).send().unwrap();
    response.status == hyper::Ok
}

fn venue_heartbeat(client: &Client, venue: String) -> bool {
    let url = format!("{}/venues/{}/heartbeat", BASE_URL, venue);
    let response = client.get(&url).send().unwrap();
    response.status == hyper::Ok
}

fn venue_stocks(client: &Client,  venue: String) -> Vec<Stock> {
    let url = format!("{}/venues/{}/stocks", BASE_URL, venue);
    let mut response = client.get(&url).send().unwrap();
    if response.status == hyper::Ok {
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        let data = Json::from_str(body.as_ref()).unwrap();
        let objs = data["symbols"].as_array().unwrap();
        objs.iter().map(|o| Stock::new(o["name"].as_string().unwrap().to_string(),
                                       o["symbol"].as_string().unwrap().to_string())).collect()
    } else {
        Vec::new()
    }
}

fn stock_orderbook(client: &Client,  venue: String, stock: String) -> OrderBook {
    let url = format!("{}/venues/{}/stocks/{}", BASE_URL, venue, stock);
    let mut response = client.get(&url).send().unwrap();
    let empty: Vec<rustc_serialize::json::Json> = Vec::new();
    if response.status == hyper::Ok {
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();
        let data = Json::from_str(body.as_ref()).unwrap();
        let bid_objs = data["bids"].as_array().unwrap_or(&empty);
        let bids: Vec<Order> = bid_objs.iter()
            .map(|o| Order::new(o["price"].as_u64().unwrap(),
                                o["qty"].as_u64().unwrap())).collect();
        let ask_objs = data["asks"].as_array().unwrap_or(&empty);
        let asks: Vec<Order> = ask_objs.iter()
            .map(|o| Order::new(o["price"].as_u64().unwrap(),
                                o["qty"].as_u64().unwrap())).collect();
        OrderBook::new(bids, asks)
    } else {
        OrderBook::new(Vec::new(), Vec::new())
    }
}

fn main() {
    println!("---StockFighter---");
    let client = Client::new();
    println!("{:?}", heartbeat(&client));
    println!("{:?}", venue_heartbeat(&client, VENUE.to_string()));
    println!("{:?}", venue_stocks(&client, VENUE.to_string()));
    println!("{:?}", stock_orderbook(&client, VENUE.to_string(), STOCK.to_string()));
}
