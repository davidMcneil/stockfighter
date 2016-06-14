use types::Direction::*;
use types::OrderType::*;
use api::*;
use std::i32;

// // Authentification for test server
// #[allow(dead_code)]
// const VENUE: &'static str = "TESTEX";
// #[allow(dead_code)]
// const SYMBOL: &'static str = "FOOBAR";
// #[allow(dead_code)]
// const ACCOUNT: &'static str = "EXB123456";

// Authentification for actual level server
#[allow(dead_code)]
const VENUE: &'static str = "CTWEX";
#[allow(dead_code)]
const SYMBOL: &'static str = "IITO";
#[allow(dead_code)]
const ACCOUNT: &'static str = "BM72606799";

macro_rules! mm_try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(_) => { println!("!!!!Continue!!!!");
        			continue; }
    });
}

pub fn run() {
	// let mut bottom_line = 0;
	let mut current_stock_count = 0;
	let target_stock_count = 100000;
	let bid_margin = 1.00f64;
	let ask_margin = 0.99f64;
	let mut bid_price = 1;
	let mut ask_price = i32::MAX;
	let bid_size = 10000;
	let ask_size = 1000;
	let mut quote;
	let mut bid = order_stock(ACCOUNT, VENUE, SYMBOL,
							  bid_price, bid_size, Buy, Limit).unwrap();
	let mut ask = order_stock(ACCOUNT, VENUE, SYMBOL,
							  ask_price, bid_size, Sell, Limit).unwrap();
	while current_stock_count < target_stock_count {
		quote = mm_try!(stock_quote(VENUE, SYMBOL));
		// Calculate my bid and ask based on margins and quote
		bid_price = (quote.bid as f64 * bid_margin) as i32;
		ask_price = (quote.ask as f64 * ask_margin) as i32;

		// Check my bid status to determine if it has been filled or if the market has moved
		bid = mm_try!(order_status(VENUE, SYMBOL, bid.id));
		if bid.price < bid_price || bid.total_filled >= bid.original_qty {
			bid = mm_try!(cancel_order(VENUE, SYMBOL, bid.id));
			current_stock_count = current_stock_count + bid.total_filled;
			bid = mm_try!(order_stock(ACCOUNT, VENUE, SYMBOL, bid_price, bid_size, Buy, Limit));
			println!("Placing bid at {}.", bid_price);
		}

		// Check my ask status to determine if it has been filled or if the market has moved
		// ask = mm_try!(order_status(VENUE, SYMBOL, ask.id));
		// if ask.price > ask_price || ask.total_filled >= ask.original_qty {
		// 	ask = mm_try!(cancel_order(VENUE, SYMBOL, ask.id));
		// 	current_stock_count = current_stock_count - ask.total_filled;
		// 	ask = mm_try!(order_stock(ACCOUNT, VENUE, SYMBOL, ask_price, ask_size, Sell, Limit));
		// 	println!("Placing sell at {}.", ask_price);
		// }
	}
}
