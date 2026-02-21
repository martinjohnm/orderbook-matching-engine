


use orderbook_rust::models::{Order,Side};
use orderbook_rust::orderbook_btreemap::OrderBookBtreeMap;

fn main() {

    let mut book = OrderBookBtreeMap::new();

    let buy_order = Order {
        id : 1, 
        symbol: String::from("BTC/USDT"),
        side: Side::Buy,
        price: 12 ,
        quantity: 123,
    };
    let sell_order = Order {
        id : 1, 
        symbol: String::from("BTC/USDT"),
        side: Side::Sell,
        price: 12 ,
        quantity: 13,
    };

    let is_buy = if let Side::Buy = buy_order.side { true } else { false };
    book.insert_order(buy_order, is_buy);

    let is_buy = if let Side::Buy = sell_order.side { true } else { false };
    book.insert_order(sell_order, is_buy);

    // let start = Instant::now();
    // // add 100k orders

    // let orders = 500_000_;
    // for i in 0..orders {
    //     let order = Order {
    //         id: i,
    //         symbol: String::from("BTC/USDT"),
    //         side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
    //         price: rand::random::<u64>() ,
    //         quantity: rand::random::<u64>() % 10 + 1,
    //     };

    //     let is_buy = if let Side::Buy = order.side { true } else { false };

    //     book.insert_order(order, is_buy);
    // }



    // let duration = start.elapsed();
    // println!("Processed 100k orders in {:?}", duration);
    // let orders_count_in_float = orders as f64;
    // println!("TPS: {}", orders_count_in_float / duration.as_secs_f64());

    println!("best bid : {:?} best ask : {:?}", book.best_bid(), book.best_ask());
}
