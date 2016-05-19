extern crate hyper;
extern crate rustc_serialize;

use types::*;
use hyper::client::response::Response;
use self::rustc_serialize::json::Json;
use std::io::Read;

const API_KEY: &'static str = "3f3c319d38a5b39acdb954c14547663f77c53cbc";
const BASE_URL: &'static str = "https://api.stockfighter.io/ob/api";
lazy_static! {
    static ref CLIENT: hyper::Client = hyper::Client::new();
}

header!{(XStarfighterAuthorization, "X-Starfighter-Authorization") => [String]}

pub fn get_headers() -> hyper::header::Headers {
    let mut headers = hyper::header::Headers::new();
    headers.set(XStarfighterAuthorization(API_KEY.to_owned()));
    headers
}

pub fn get_response_json(response: & mut Response, err_msg: &str) -> Json {
    let mut response_body = String::new();
    response.read_to_string(&mut response_body).expect(&err_msg);
    Json::from_str(&response_body).expect(&err_msg)
}

pub fn json_to_string(json: &Json, err_msg: &str) -> String {
    json.as_string().expect(err_msg).to_string()
}

pub fn json_to_u64(json: &Json, err_msg: &str) -> u64 {
    json.as_u64().expect(err_msg)
}

#[allow(dead_code)]
pub fn heartbeat() -> bool {
    let url = format!("{}/heartbeat", BASE_URL);
    let result = CLIENT.get(&url).send();
    match result {
        Ok(response) => (response.status == hyper::Ok),
        Err(_) => false,
    }
}

#[allow(dead_code)]
pub fn venue_heartbeat(venue: &str) -> bool {
    let url = format!("{}/venues/{}/heartbeat", BASE_URL, venue);
    let result = CLIENT.get(&url).send();
    match result {
        Ok(response) => (response.status == hyper::Ok),
        Err(_) => false,
    }
}

#[allow(dead_code)]
pub fn venue_stocks(venue: &str) -> Vec<Stock> {
    let url = format!("{}/venues/{}/stocks", BASE_URL, venue);
    let err_msg = format!("Unable to retrive stocks from venue {}", venue);
    let mut response = CLIENT.get(&url)
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let response_json = get_response_json(& mut response, &err_msg);
    let stock_objs = response_json["symbols"].as_array().expect(&err_msg);
    stock_objs.iter().map(|o| Stock::new(json_to_string(&o["name"], &err_msg),
                                         json_to_string(&o["symbol"], &err_msg))).collect()
}

#[allow(dead_code)]
pub fn stock_orderbook(venue: &str, stock: &str) -> OrderBook {
    let url = format!("{}/venues/{}/stocks/{}", BASE_URL, venue, stock);
    let err_msg = format!("Unable to retrive orderbook for stock {} on venue {}", stock, venue);
    let mut response = CLIENT.get(&url)
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let empty: Vec<rustc_serialize::json::Json> = Vec::new();
    let response_json = get_response_json(& mut response, &err_msg);
    let bid_objs = response_json["bids"].as_array().unwrap_or(&empty);
    let ask_objs = response_json["asks"].as_array().unwrap_or(&empty);
    let bids: Vec<Order> = bid_objs.iter().map(|o|
        Order::new(json_to_u64(&o["price"], &err_msg),
                    json_to_u64(&o["qty"], &err_msg))).collect();
    let asks: Vec<Order> = ask_objs.iter().map(|o|
        Order::new(json_to_u64(&o["price"], &err_msg),
                   json_to_u64(&o["qty"], &err_msg))).collect();
    OrderBook::new(venue.to_string(), stock.to_string(), bids, asks)
}

#[allow(dead_code)]
pub fn order_stock(account: &str, venue: &str, stock: &str, price: u64, qty: u64,
                   direction: Direction, order_type: OrderType) -> OrderResult {
    let url = format!("{}/venues/{}/stocks/{}/orders", BASE_URL, venue, stock);
    let err_msg = format!("Unable to place order for stock {} on venue {}", stock, venue);
    let stock_order = StockOrder::new(account.to_string(), venue.to_string(), stock.to_string(),
                                      price, qty, direction.clone(), order_type);
    let encoded = rustc_serialize::json::encode(&stock_order).expect(&err_msg);
    let mut response = CLIENT.post(&url)
        .headers(get_headers())
        .body(&encoded)
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let empty: Vec<rustc_serialize::json::Json> = Vec::new();
    let response_json = get_response_json(& mut response, &err_msg);
    let fill_objs = response_json["fills"].as_array().unwrap_or(&empty);
    let fills: Vec<Fill> = fill_objs.iter().map(|o|
        Fill::new(json_to_u64(&o["price"], &err_msg),
                  json_to_u64(&o["qty"], &err_msg))).collect();
    let total_filled = json_to_u64(&response_json["totalFilled"], &err_msg);
    let id = json_to_u64(&response_json["id"], &err_msg);
    OrderResult::new(id, venue.to_string(), stock.to_string(), direction, qty, total_filled, fills)
}

#[allow(dead_code)]
pub fn stock_quote(venue: &str, stock: &str) -> StockQuote {
    let url = format!("{}/venues/{}/stocks/{}/quote", BASE_URL, venue, stock);
    let err_msg = format!("Unable to get stock quote for stock {} on venue {}", stock, venue);
    let mut response = CLIENT.get(&url)
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let response_json = get_response_json(& mut response, &err_msg);
    println!("{:?}", response_json);
    let bid_size = json_to_u64(&response_json["bidSize"], &err_msg);
    let ask_size = json_to_u64(&response_json["askSize"], &err_msg);
    let bid_depth = json_to_u64(&response_json["bidDepth"], &err_msg);
    let ask_depth = json_to_u64(&response_json["askDepth"], &err_msg);
    let bid = if bid_depth > 0 { json_to_u64(&response_json["bid"], &err_msg) } else { 0 };
    let ask = if ask_depth > 0 { json_to_u64(&response_json["ask"], &err_msg) } else { 0 };
    let last = json_to_u64(&response_json["last"], &err_msg);
    let last_size = json_to_u64(&response_json["lastSize"], &err_msg);
    StockQuote::new(venue.to_string(), stock.to_string(),
                    bid, ask, bid_size, ask_size, bid_depth, ask_depth, last, last_size)
}
