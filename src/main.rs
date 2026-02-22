


use std::time::Instant;

use orderbook_rust::models::{Order,Side};
use orderbook_rust::orderbook_btreemap::OrderBookBtreeMap;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
fn main() {

    let mut book = OrderBookBtreeMap::new();

    // let buy_order = Order {
    //     id : 1, 
    //     symbol: String::from("BTC/USDT"),
    //     side: Side::Buy,
    //     price: 12 ,
    //     quantity: 123,
    // };
    // let sell_order = Order {
    //     id : 1, 
    //     symbol: String::from("BTC/USDT"),
    //     side: Side::Sell,
    //     price: 12 ,
    //     quantity: 13,
    // };

    // let is_buy = if let Side::Buy = buy_order.side { true } else { false };
    // book.insert_order(buy_order, is_buy);

    // let is_buy = if let Side::Buy = sell_order.side { true } else { false };
    // book.insert_order(sell_order, is_buy);

    let start = Instant::now();
    // add 100k orders

    let mut rng = SmallRng::from_entropy();
    let orders = 500_0000;
    
    for i in 0..orders {

        

        let mut order = Order {
            id: i,
            symbol: String::from("BTC/USDT"),
            side: if i % 2 == 0 { Side::Buy } else { Side::Sell },
            price: rng.gen_range(1000..1010) ,
            quantity: rand::random::<u64>() % 10 + 1,
        };

        let is_buy = if let Side::Buy = order.side { true } else { false };

        let price = if is_buy {
            rng.gen_range(1005..1050) // Buyers willing to pay more
        } else {
            rng.gen_range(970..1010) // Sellers willing to take less
        };

        order.price = price;

        book.insert_order(order, is_buy);
    }



    let duration = start.elapsed();
    println!("Processed {} orders in {:?}",orders,  duration);
    let orders_count_in_float = orders as f64;
    println!("TPS: {}", orders_count_in_float / duration.as_secs_f64());

    // println!("best bid : {:?} best ask : {:?}", book.best_bid(), book.best_ask());


    println!("No of Bids : {}", book.total_bid_count());
    println!("No of asks : {}", book.total_ask_count());

}

