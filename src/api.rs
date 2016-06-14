extern crate hyper;
extern crate rustc_serialize;

use types::*;
use rustc_serialize::json::Json;
use std::io::Read;
use std::i32;

const API_KEY: &'static str = "3f3c319d38a5b39acdb954c14547663f77c53cbc";
const BASE_URL: &'static str = "https://api.stockfighter.io/ob/api";
lazy_static! {
    static ref CLIENT: hyper::Client = hyper::Client::new();
}

header!{(XStarfighterAuthorization, "X-Starfighter-Authorization") => [String]}

#[derive(Debug)]
pub struct ApiError;

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    fn from<T, E>(result: Result<T, E>) -> Result<T, ApiError> {
        match result {
            Ok(value) => Ok(value),
            _ => Err(ApiError),
        }
    }
}

macro_rules! api_try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return ApiError::from(Err(err)),
    });
}

macro_rules! unpack {
    ($e:expr) => (match $e {
        Some(val) => val,
        None => return Err(ApiError),
    });
}

fn get_response_json<'a>(request_builder: hyper::client::RequestBuilder<'a>,
                         body: &'a str) -> ApiResult<Json> {
    let mut headers = hyper::header::Headers::new();
    headers.set(XStarfighterAuthorization(API_KEY.to_owned()));
    let mut response = api_try!(request_builder
        .headers(headers)
        .body(body)
        .send());
    if response.status != hyper::Ok { return Err(ApiError) };
    let mut response_body = String::new();
    api_try!(response.read_to_string(&mut response_body));
    Ok(api_try!(Json::from_str(&response_body)))
}

fn json_lookup_string(json: &Json, key: &str) -> ApiResult<String> {
    let btree = unpack!(json.as_object());
    if btree.contains_key(key) {
        let val = &btree[key];
        Ok(unpack!(val.as_string()).to_string())
    } else {
        Err(ApiError)
    }
}

fn json_lookup_i32(json: &Json, key: &str) -> ApiResult<i32> {
    let btree = unpack!(json.as_object());
    if btree.contains_key(key) {
        let val = &btree[key];
        Ok(unpack!(val.as_i64()) as i32)
    } else {
        Err(ApiError)
    }
}

fn json_lookup_array(json: &Json, key: &str) -> ApiResult<Vec<Json>> {
    let btree = unpack!(json.as_object());
    if btree.contains_key(key) {
        let val = &btree[key];
        let array = unpack!(val.as_array()).clone(); 
        Ok(array)
    } else {
        Err(ApiError)
    }
}

fn json_to_order_status(json: &Json) -> ApiResult<OrderStatus> {
    let id = api_try!(json_lookup_i32(&json, "id"));
    let venue = api_try!(json_lookup_string(&json, "venue"));
    let symbol = api_try!(json_lookup_string(&json, "symbol"));
    let direction = api_try!(json_lookup_string(&json, "direction"));
    let direction = Direction::decode(direction);
    let original_qty = api_try!(json_lookup_i32(&json, "originalQty"));
    let total_filled = api_try!(json_lookup_i32(&json, "totalFilled"));
    let price = api_try!(json_lookup_i32(&json, "price"));
    let fill_objs = api_try!(json_lookup_array(&json, "fills"));
    let order_type = api_try!(json_lookup_string(&json, "orderType"));
    let order_type = OrderType::decode(order_type);
    let fills: Vec<Fill> = api_try!(fill_objs.iter().map(|o|
        Ok(Fill::new(api_try!(json_lookup_i32(&o, "price")),
                     api_try!(json_lookup_i32(&o, "qty"))))).collect());
    Ok(OrderStatus::new(id, venue, symbol,
        direction, original_qty, total_filled, price, order_type, fills))
}

#[allow(dead_code)]
pub fn heartbeat() -> bool {
    let url = format!("{}/heartbeat", BASE_URL);
    let result = CLIENT.get(&url).send();
    match result {
        Ok(response) => (response.status == hyper::Ok),
        Err(_) => false
    }
}

#[allow(dead_code)]
pub fn venue_heartbeat(venue: &str) -> bool {
    let url = format!("{}/venues/{}/heartbeat", BASE_URL, venue);
    let result = CLIENT.get(&url).send();
    match result {
        Ok(response) => (response.status == hyper::Ok),
        Err(_) => false
    }
}

#[allow(dead_code)]
pub fn venue_stocks(venue: &str) -> ApiResult<Vec<Stock>> {
    let url = format!("{}/venues/{}/stocks", BASE_URL, venue);
    let json = api_try!(get_response_json(CLIENT.get(&url), ""));
    let stock_objs = api_try!(json_lookup_array(&json, "symbols"));
    stock_objs.iter().map(|o| Ok(Stock::new(api_try!(json_lookup_string(&o, "name")),
                                            api_try!(json_lookup_string(&o, "symbol"))))).collect()
}

#[allow(dead_code)]
pub fn stock_orderbook(venue: &str, symbol: &str) -> ApiResult<OrderBook> {
    let url = format!("{}/venues/{}/stocks/{}", BASE_URL, venue, symbol);
    let json = api_try!(get_response_json(CLIENT.get(&url), ""));
    let venue = api_try!(json_lookup_string(&json, "venue"));
    let symbol = api_try!(json_lookup_string(&json, "symbol"));
    let bid_objs = api_try!(json_lookup_array(&json, "bids"));
    let ask_objs = api_try!(json_lookup_array(&json, "asks"));
    let bids: Vec<Order> = api_try!(bid_objs.iter().map(|o|
        Ok(Order::new(api_try!(json_lookup_i32(&o, "price")),
                      api_try!(json_lookup_i32(&o, "qty"))))).collect());
    let asks: Vec<Order> = api_try!(ask_objs.iter().map(|o|
        Ok(Order::new(api_try!(json_lookup_i32(&o, "price")),
                      api_try!(json_lookup_i32(&o, "qty"))))).collect());
    Ok(OrderBook::new(venue, symbol, bids, asks))
}

#[allow(dead_code)]
pub fn stock_quote(venue: &str, symbol: &str) -> ApiResult<StockQuote> {
    let url = format!("{}/venues/{}/stocks/{}/quote", BASE_URL, venue, symbol);
    let json = api_try!(get_response_json(CLIENT.get(&url), ""));
    let venue = api_try!(json_lookup_string(&json, "venue"));
    let symbol = api_try!(json_lookup_string(&json, "symbol"));
    let bid_size = api_try!(json_lookup_i32(&json, "bidSize"));
    let ask_size = api_try!(json_lookup_i32(&json, "askSize"));
    let bid_depth = api_try!(json_lookup_i32(&json, "bidDepth"));
    let ask_depth = api_try!(json_lookup_i32(&json, "askDepth"));
    let bid = api_try!(if bid_depth > 0 { json_lookup_i32(&json, "bid") }
                       else { Ok(0) });
    let ask = api_try!(if ask_depth > 0 { json_lookup_i32(&json, "ask") } 
                       else { Ok(i32::MAX) });
    let last = api_try!(json_lookup_i32(&json, "last"));
    let last_size = api_try!(json_lookup_i32(&json, "lastSize"));
    Ok(StockQuote::new(venue, symbol, bid, ask, bid_size, ask_size,
                    bid_depth, ask_depth, last, last_size))
}

#[allow(dead_code)]
pub fn order_stock(account: &str, venue: &str, symbol: &str, price: i32, qty: i32,
                   direction: Direction, order_type: OrderType) -> ApiResult<OrderStatus> {
    let url = format!("{}/venues/{}/stocks/{}/orders", BASE_URL, venue, symbol);
    let stock_order = StockOrder::new(account.to_string(), venue.to_string(), symbol.to_string(),
                                      price, qty, direction.clone(), order_type);
    let encoded = api_try!(rustc_serialize::json::encode(&stock_order));
    let json = api_try!(get_response_json(CLIENT.post(&url), &encoded));
    json_to_order_status(&json)
}

#[allow(dead_code)]
pub fn cancel_order(venue: &str, symbol: &str, id: i32) -> ApiResult<OrderStatus> {
    let url = format!("{}/venues/{}/stocks/{}/orders/{}", BASE_URL, venue, symbol, id);
    let json = api_try!(get_response_json(CLIENT.delete(&url), ""));
    json_to_order_status(&json)
}

#[allow(dead_code)]
pub fn order_status(venue: &str, symbol: &str, id: i32) -> ApiResult<OrderStatus> {
    let url = format!("{}/venues/{}/stocks/{}/orders/{}", BASE_URL, venue, symbol, id);
    let json = api_try!(get_response_json(CLIENT.get(&url), ""));
    json_to_order_status(&json)
}


#[allow(dead_code)]
pub fn venue_order_statuses(venue: &str, account: &str) -> ApiResult<Vec<OrderStatus>> {
    let url = format!("{}/venues/{}/accounts/{}/orders", BASE_URL, venue, account);
    let json = api_try!(get_response_json(CLIENT.get(&url), ""));
    let orders = api_try!(json_lookup_array(&json, "orders"));
    orders.iter().map(|o| json_to_order_status(&o)).collect()
}

#[allow(dead_code)]
pub fn stock_order_statuses(venue: &str, account: &str, symbol: &str)
    -> ApiResult<Vec<OrderStatus>> {
    let url = format!("{}/venues/{}/accounts/{}/stocks/{}/orders",
                       BASE_URL, venue, account, symbol);
    let json = api_try!(get_response_json(CLIENT.get(&url), ""));
    let orders = api_try!(json_lookup_array(&json, "orders"));
    orders.iter().map(|o| json_to_order_status(&o)).collect()
}
