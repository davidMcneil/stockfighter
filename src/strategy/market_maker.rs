use types::*;
use types::Direction::*;
use types::OrderType::*;
use api::*;
use std::i32;

// Authentification for actual level server
#[allow(dead_code)]
const VENUE: &'static str = "YMNEX";
#[allow(dead_code)]
const SYMBOL: &'static str = "UPF";
#[allow(dead_code)]
const ACCOUNT: &'static str = "SLB5120371";

// // Authentification for test server
// #[allow(dead_code)]
// const VENUE: &'static str = "TESTEX";
// #[allow(dead_code)]
// const SYMBOL: &'static str = "FOOBAR";
// #[allow(dead_code)]
// const ACCOUNT: &'static str = "EXB123456";

pub fn run() {
	let mut bottom_line = 0u64;
	let mut current_stock_count = 0u64;
	let target_stock_count = 10000u64;
	let bid_margin = 1.0f64;
	let ask_margin = 1.05f64;
	let mut bid_price = 1u64;
	let mut ask_price = i32::MAX as u64;
	let bid_size = 1000u64;
	let ask_size = 1000u64;
	let mut quote;
	let mut bid = order_stock(ACCOUNT, VENUE, SYMBOL, bid_price, bid_size, Buy, Limit);
	let mut ask = order_stock(ACCOUNT, VENUE, SYMBOL, ask_price, bid_size, Sell, Limit);
	while current_stock_count < target_stock_count {
		// Get quote from market
		quote = stock_quote(VENUE, SYMBOL);
		// Calculate my margins on the quote
		bid_price = (quote.bid as f64 * bid_margin) as u64;
		ask_price = (quote.ask as f64 * ask_margin) as u64;
		// Get my current bid status
		bid = order_status(VENUE, SYMBOL, bid.id);
		// Check my bid status against market quote
		if bid.price < bid_price || bid.total_filled >= bid.original_qty {
			bid = cancel_order(VENUE, SYMBOL, bid.id);
			// current_stock_count = current_stock_count + bid.total_filled;
			bid = order_stock(ACCOUNT, VENUE, SYMBOL, bid_price, bid_size, Buy, Limit);
			println!("Placing bid at {}.", bid_price);
		}
		// Get my current ask status
		ask = order_status(VENUE, SYMBOL, ask.id);
		// Check my ask status against market quote
		if ask.price > ask_price || ask.total_filled >= ask.original_qty {
			ask = cancel_order(VENUE, SYMBOL, ask.id);
			// current_stock_count = current_stock_count - bid.total_filled;
			ask = order_stock(ACCOUNT, VENUE, SYMBOL, ask_price, ask_size, Sell, Limit);
			println!("Placing sell at {}.", ask_price);
		}
	}
}
