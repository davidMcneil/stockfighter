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
    pub price: i32,
    pub qty: i32
}

impl Order {
    pub fn new(price: i32, qty: i32) -> Order {
        Order{ price: price, qty: qty }
    }
}

#[derive(Debug)]
pub struct Fill {
    price: i32,
    qty: i32
}

impl Fill {
    pub fn new(price: i32, qty: i32) -> Fill {
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
 #[derive(Debug, Clone, RustcEncodable)]
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
  #[derive(Debug, Clone, RustcEncodable)]
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
    pub price: i32,
    pub qty: i32,
    pub direction: Direction,
    pub order_type: OrderType
}

impl StockOrder {
    pub fn new(account: String, venue: String, symbol: String, price: i32,
               qty: i32, direction: Direction, order_type: OrderType) -> StockOrder {
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
pub struct OrderStatus {
    pub id: i32,
    pub venue: String,
    pub symbol: String,
    pub direction: Direction,
    pub original_qty: i32,
    pub total_filled: i32,
    pub price: i32,
    pub order_type: OrderType,
    pub fills: Vec<Fill>
}

impl OrderStatus {
    pub fn new(id: i32, venue: String, symbol: String, direction: Direction, original_qty: i32,
               total_filled: i32, price: i32, order_type: OrderType, fills: Vec<Fill>) -> OrderStatus {
        OrderStatus{ id: id, venue: venue, symbol: symbol, direction: direction,
                     original_qty: original_qty, total_filled: total_filled, price: price, order_type: order_type,
                     fills: fills }
    }
}

#[derive(Debug)]
pub struct StockQuote {
    pub venue: String,
    pub symbol: String,
    pub bid: i32,       // best price currently bid for the stock
    pub ask: i32,       // best price currently offered for the stock
    pub bid_size: i32,  // aggregate size of all orders at the best bid
    pub ask_size: i32,  // aggregate size of all orders at the best ask
    pub bid_depth: i32, // aggregate size of *all bids*
    pub ask_depth: i32, // aggregate size of *all asks*
    pub last: i32,      // price of last trade
    pub last_size: i32  // quantity of last trade
}

impl StockQuote {
    pub fn new(venue: String, symbol: String, bid: i32, ask: i32, bid_size: i32, ask_size: i32,
               bid_depth: i32, ask_depth: i32, last: i32, last_size: i32) -> StockQuote {
        StockQuote{ venue: venue, symbol: symbol, bid: bid, ask: ask,
                    bid_size: bid_size, ask_size: ask_size, bid_depth: bid_depth,
                    ask_depth: ask_depth, last: last, last_size: last_size }
    }
}
