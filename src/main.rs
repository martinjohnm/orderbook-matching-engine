

use orderbook_rust::models::{Order,Side};
use orderbook_rust::orderbook::OrderBook;

fn main() {

    let mut book = OrderBook::new();

    book.add_order(Order { id: 1, symbol: String::from("BTC/USDT"), side: Side::Buy, price: 100.0, quantity: 10 });
    let trades2 = book.add_order(Order { id: 2, symbol: String::from("BTC/USDT"), side: Side::Sell, price: 95.0, quantity: 5 });
    
    println!("Trades after second order : {:?}", trades2);

    let trades3 = book.add_order(Order { id: 3, symbol: String::from("BTC/USDT"), side: Side::Buy, price: 105.0, quantity: 5 });

    println!("Trades after third order: {:?}", trades3);
}
