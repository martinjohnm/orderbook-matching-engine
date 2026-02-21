

use std::time::Instant;

use orderbook_rust::models::{Order,Side};
use orderbook_rust::orderbook::OrderBook;

fn main() {

    let mut book = OrderBook::new();

    book.add_order(Order { id: 1, symbol: String::from("BTC/USDT"), side: Side::Buy, price: 100.0, quantity: 10 });
    let trades2 = book.add_order(Order { id: 2, symbol: String::from("BTC/USDT"), side: Side::Sell, price: 95.0, quantity: 5 });
    
    println!("Trades after second order : {:?}", trades2);

    let trades3 = book.add_order(Order { id: 3, symbol: String::from("BTC/USDT"), side: Side::Buy, price: 105.0, quantity: 4 });

    println!("Trades after third order: {:?}", trades3);

    

    let start = Instant::now();
    // add 100k orders
    for i in 0..100_000 {
        let order = Order {
            id: i,
            symbol: String::from("BTC/USDT"),
            side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
            price: rand::random::<f64>() * 100.0,
            quantity: rand::random::<u64>() % 10 + 1,
        };
        book.add_order(order);
    }
    let duration = start.elapsed();
    println!("Processed 100k orders in {:?}", duration);
    println!("TPS: {}", 100_000.0 / duration.as_secs_f64());
}
