

use orderbook_rust::models::{Order, order::Side};

fn main() {


    let sid = Side::Buy;

    let o = Order {
        id : 34,
        symbol : String::from("BTC/USDT"),
        side : sid,
        price: 45.0,
        quantity : 100
    };

    println!("{:?}", o);

}
