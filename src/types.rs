extern crate rustc_serialize;
use self::rustc_serialize::{ Encodable, Encoder };
use std::ascii::AsciiExt;

#[derive(Debug)]
pub struct Stock {
    pub name: String,
    pub symbol: String
}

impl Stock {
    pub fn new(name: String, symbol: String) -> Stock {
        Stock{ name: name, symbol: symbol }
    }
}

#[derive(Debug)]
pub struct Order {
    pub price: u64,
    pub qty: u64
}

impl Order {
    pub fn new(price: u64, qty: u64) -> Order {
        Order{ price: price, qty: qty }
    }
}

#[derive(Debug)]
pub struct Fill {
    price: u64,
    qty: u64
}

impl Fill {
    pub fn new(price: u64, qty: u64) -> Fill {
        Fill{ price: price, qty: qty }
    }
}

#[derive(Debug)]
pub struct OrderBook {
    pub venue: String,
    pub symbol: String,
    pub bids: Vec<Order>,
    pub asks: Vec<Order>
}

impl OrderBook {
    pub fn new(venue: String, symbol: String, bids: Vec<Order>, asks: Vec<Order>) -> OrderBook {
        OrderBook{ venue: venue, symbol: symbol, bids: bids, asks: asks }
    }
}

#[allow(dead_code)] #[derive(Debug, Clone, RustcEncodable)]
pub enum Direction {
    Buy,
    Sell
}

impl Direction {
    pub fn decode(string: String) -> Direction {
        match string.trim().to_ascii_lowercase().as_ref() {
            "buy" => Direction::Buy,
            "sell" => Direction::Sell,
            _ => panic!("Unexpected Direction"),
        }
    }
}

#[allow(dead_code)]  #[derive(Debug, Clone, RustcEncodable)]
pub enum OrderType {
    Limit,
    Market,
    Fok,
    Ioc
}

impl OrderType {
    pub fn decode(string: String) -> OrderType {
        match string.trim().to_ascii_lowercase().as_ref() {
            "limit" => OrderType::Limit,
            "market" => OrderType::Market,
            "fok" => OrderType::Fok,
            "fill-or-kill" => OrderType::Fok,
            "ioc" => OrderType::Ioc,
            "immediate-or-cancel" => OrderType::Ioc,
            _ => panic!("Unexpected OrderType"),
        }
    }
}

#[derive(Debug)]
pub struct StockOrder {
    pub account: String,
    pub venue: String,
    pub symbol: String,
    pub price: u64,
    pub qty: u64,
    pub direction: Direction,
    pub order_type: OrderType
}

impl StockOrder {
    pub fn new(account: String, venue: String, symbol: String, price: u64,
               qty: u64, direction: Direction, order_type: OrderType) -> StockOrder {
        StockOrder{ account: account, venue: venue, symbol: symbol, price: price,
                    qty: qty, direction: direction, order_type: order_type }
    }
}

impl Encodable for StockOrder {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("StockOrder", 7, |e| {
            try!(e.emit_struct_field("account", 0, |e| self.account.encode(e)));
            try!(e.emit_struct_field("venue", 1, |e| self.venue.encode(e)));
            try!(e.emit_struct_field("stock", 2, |e| self.symbol.encode(e)));
            try!(e.emit_struct_field("price", 3, |e| self.price.encode(e)));
            try!(e.emit_struct_field("qty", 4, |e| self.qty.encode(e)));
            try!(e.emit_struct_field("direction", 5, |e| self.direction.encode(e)));
            e.emit_struct_field("orderType", 6, |e| self.order_type.encode(e))
        })
    }
}

#[derive(Debug)]
pub struct OrderResult {
    pub id: u64,
    pub venue: String,
    pub symbol: String,
    pub direction: Direction,
    pub original_qty: u64,
    pub total_filled: u64,
    pub fills: Vec<Fill>
}

impl OrderResult {
    pub fn new(id: u64, venue: String, symbol: String, direction: Direction,
               original_qty: u64, total_filled: u64, fills: Vec<Fill>) -> OrderResult {
        OrderResult{ id: id, venue: venue, symbol: symbol, direction: direction,
                     original_qty: original_qty, total_filled: total_filled, fills: fills }
    }
}

#[derive(Debug)]
pub struct StockQuote {
    pub venue: String,
    pub symbol: String,
    pub bid: u64,       // best price currently bid for the stock
    pub ask: u64,       // best price currently offered for the stock
    pub bid_size: u64,  // aggregate size of all orders at the best bid
    pub ask_size: u64,  // aggregate size of all orders at the best ask
    pub bid_depth: u64, // aggregate size of *all bids*
    pub ask_depth: u64, // aggregate size of *all asks*
    pub last: u64,      // price of last trade
    pub last_size: u64  // quantity of last trade
}

impl StockQuote {
    pub fn new(venue: String, symbol: String, bid: u64, ask: u64, bid_size: u64, ask_size: u64,
               bid_depth: u64, ask_depth: u64, last: u64, last_size: u64) -> StockQuote {
        StockQuote{ venue: venue, symbol: symbol, bid: bid, ask: ask,
                    bid_size: bid_size, ask_size: ask_size, bid_depth: bid_depth,
                    ask_depth: ask_depth, last: last, last_size: last_size }
    }
}

#[derive(Debug)]
pub struct OrderStatus {
    pub id: u64,
    pub venue: String,
    pub symbol: String,
    pub direction: Direction,
    pub original_qty: u64,
    pub total_filled: u64,
    pub price: u64,
    pub order_type: OrderType,
    pub fills: Vec<Fill>
}

impl OrderStatus {
    pub fn new(id: u64, venue: String, symbol: String, direction: Direction, original_qty: u64,
               total_filled: u64, price: u64, order_type: OrderType, fills: Vec<Fill>) -> OrderStatus {
        OrderStatus{ id: id, venue: venue, symbol: symbol, direction: direction,
                     original_qty: original_qty, total_filled: total_filled, price: price, order_type: order_type,
                     fills: fills }
    }
}

