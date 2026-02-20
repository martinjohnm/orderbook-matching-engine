

use orderbook_rust::models::{Order,Side};

fn main() {


    let side = Side::Buy;

    let o = Order {
        id : 34,
        symbol : String::from("BTC/USDT"),
        side ,
        price: 45.0,
        quantity : 100
    };

    println!("{:?}", o);

}
