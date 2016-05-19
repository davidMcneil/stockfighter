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

pub fn json_to_order_status(json: &Json, err_msg: &str) -> OrderStatus {
    let empty: Vec<rustc_serialize::json::Json> = Vec::new();
    let id = json_to_u64(&json["id"], &err_msg);
    let venue = json_to_string(&json["venue"], &err_msg);
    let symbol = json_to_string(&json["symbol"], &err_msg);
    let direction = Direction::decode(json_to_string(&json["direction"], &err_msg));
    let original_qty = json_to_u64(&json["originalQty"], &err_msg);
    let total_filled = json_to_u64(&json["totalFilled"], &err_msg);
    let price = json_to_u64(&json["price"], &err_msg);
    let fill_objs = json["fills"].as_array().unwrap_or(&empty);
    let order_type = OrderType::decode(json_to_string(&json["orderType"], &err_msg));
    let fills: Vec<Fill> = fill_objs.iter().map(|o|
        Fill::new(json_to_u64(&o["price"], &err_msg),
                  json_to_u64(&o["qty"], &err_msg))).collect();
    OrderStatus::new(id, venue, symbol,
                     direction, original_qty, total_filled, price, order_type, fills)
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
pub fn stock_orderbook(venue: &str, symbol: &str) -> OrderBook {
    let url = format!("{}/venues/{}/stocks/{}", BASE_URL, venue, symbol);
    let err_msg = format!("Unable to retrive orderbook for stock {} on venue {}", symbol, venue);
    let mut response = CLIENT.get(&url)
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let empty: Vec<rustc_serialize::json::Json> = Vec::new();
    let response_json = get_response_json(& mut response, &err_msg);
    let venue = json_to_string(&response_json["venue"], &err_msg);
    let symbol = json_to_string(&response_json["symbol"], &err_msg);
    let bid_objs = response_json["bids"].as_array().unwrap_or(&empty);
    let ask_objs = response_json["asks"].as_array().unwrap_or(&empty);
    let bids: Vec<Order> = bid_objs.iter().map(|o|
        Order::new(json_to_u64(&o["price"], &err_msg),
                    json_to_u64(&o["qty"], &err_msg))).collect();
    let asks: Vec<Order> = ask_objs.iter().map(|o|
        Order::new(json_to_u64(&o["price"], &err_msg),
                   json_to_u64(&o["qty"], &err_msg))).collect();
    OrderBook::new(venue, symbol, bids, asks)
}

#[allow(dead_code)]
pub fn order_stock(account: &str, venue: &str, symbol: &str, price: u64, qty: u64,
                   direction: Direction, order_type: OrderType) -> OrderStatus {
    let url = format!("{}/venues/{}/stocks/{}/orders", BASE_URL, venue, symbol);
    let err_msg = format!("Unable to place order for stock {} on venue {}", symbol, venue);
    let stock_order = StockOrder::new(account.to_string(), venue.to_string(), symbol.to_string(),
                                      price, qty, direction.clone(), order_type);
    let encoded = rustc_serialize::json::encode(&stock_order).expect(&err_msg);
    let mut response = CLIENT.post(&url)
        .headers(get_headers())
        .body(&encoded)
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let response_json = get_response_json(& mut response, &err_msg);
    json_to_order_status(&response_json, &err_msg)
}

#[allow(dead_code)]
pub fn stock_quote(venue: &str, symbol: &str) -> StockQuote {
    let url = format!("{}/venues/{}/stocks/{}/quote", BASE_URL, venue, symbol);
    let err_msg = format!("Unable to get stock quote for stock {} on venue {}", symbol, venue);
    let mut response = CLIENT.get(&url)
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let response_json = get_response_json(& mut response, &err_msg);
    println!("{:?}", response_json);
    let venue = json_to_string(&response_json["venue"], &err_msg);
    let symbol = json_to_string(&response_json["symbol"], &err_msg);
    let bid_size = json_to_u64(&response_json["bidSize"], &err_msg);
    let ask_size = json_to_u64(&response_json["askSize"], &err_msg);
    let bid_depth = json_to_u64(&response_json["bidDepth"], &err_msg);
    let ask_depth = json_to_u64(&response_json["askDepth"], &err_msg);
    let bid = if bid_depth > 0 { json_to_u64(&response_json["bid"], &err_msg) } else { 0 };
    let ask = if ask_depth > 0 { json_to_u64(&response_json["ask"], &err_msg) } else { 0 };
    let last = json_to_u64(&response_json["last"], &err_msg);
    let last_size = json_to_u64(&response_json["lastSize"], &err_msg);
    StockQuote::new(venue, symbol, bid, ask, bid_size, ask_size,
                    bid_depth, ask_depth, last, last_size)
}

#[allow(dead_code)]
pub fn order_status(venue: &str, symbol: &str, id: u64) -> OrderStatus {
    let url = format!("{}/venues/{}/stocks/{}/orders/{}", BASE_URL, venue, symbol, id);
    let err_msg = format!("Unable to get status of order {} for stock {} on venue {}",
                           id, symbol, venue);
    let mut response = CLIENT.get(&url)
        .headers(get_headers())
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let response_json = get_response_json(& mut response, &err_msg);
    json_to_order_status(&response_json, &err_msg)
}

#[allow(dead_code)]
pub fn cancel_order(venue: &str, symbol: &str, id: u64) -> OrderStatus {
    let url = format!("{}/venues/{}/stocks/{}/orders/{}", BASE_URL, venue, symbol, id);
    let err_msg = format!("Unable to cancel order {} for stock {} on venue {}", id, symbol, venue);
    let mut response = CLIENT.delete(&url)
        .headers(get_headers())
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let response_json = get_response_json(& mut response, &err_msg);
    json_to_order_status(&response_json, &err_msg)
}

#[allow(dead_code)]
pub fn venue_order_statuses(venue: &str, account: &str) -> Vec<OrderStatus> {
    let url = format!("{}/venues/{}/accounts/{}/orders", BASE_URL, venue, account);
    let err_msg = format!("Unable to get venue {} order statuses", venue);
    let mut response = CLIENT.get(&url)
        .headers(get_headers())
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let empty: Vec<rustc_serialize::json::Json> = Vec::new();
    let response_json = get_response_json(& mut response, &err_msg);
    let orders = response_json["orders"].as_array().unwrap_or(&empty);
    orders.iter().map(|o| json_to_order_status(&o, &err_msg)).collect()
}

#[allow(dead_code)]
pub fn stock_order_statuses(venue: &str, account: &str, symbol: &str) -> Vec<OrderStatus> {
    let url = format!("{}/venues/{}/accounts/{}/stocks/{}/orders",
                       BASE_URL, venue, account, symbol);
    let err_msg = format!("Unable to get venue {} order statuses", venue);
    let mut response = CLIENT.get(&url)
        .headers(get_headers())
        .send()
        .expect(&err_msg);
    assert!(response.status == hyper::Ok, err_msg);
    let empty: Vec<rustc_serialize::json::Json> = Vec::new();
    let response_json = get_response_json(& mut response, &err_msg);
    let orders = response_json["orders"].as_array().unwrap_or(&empty);
    orders.iter().map(|o| json_to_order_status(&o, &err_msg)).collect()
}
