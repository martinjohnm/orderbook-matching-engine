



use crate::models::{Order, Side, Trade};

#[derive(Debug)]
pub struct OrderBook {
    buy : Vec<Order>, // sorted descending by price
    sell : Vec<Order> // sorted ascending by price
}

impl OrderBook {
    pub fn new() -> Self {
        Self { buy: vec![], sell: vec![] }
    }

    pub fn add_order(&mut self, order: Order) -> Vec<Trade> {
        let book = match order.side {
            Side::Buy => &mut self.buy,
            Side::Sell => &mut self.sell
        };

        book.push(order);
        
        // Sort arrays 
        self.buy.sort_by(|a,b| b.price.partial_cmp(&a.price).unwrap());
        self.sell.sort_by(|a,b| a.price.partial_cmp(&b.price).unwrap());

        self.match_orders()

    }

    pub fn match_orders(&mut self) -> Vec<Trade> {
        let mut trades = vec![];

        while !self.buy.is_empty() && !self.sell.is_empty() && self.buy[0].price >= self.sell[0].price {
            let mut buy_order = self.buy.remove(0);
            let mut sell_order = self.sell.remove(0);
            let qty = buy_order.quantity.min(sell_order.quantity);

            trades.push(Trade { 
                buy_order_id: buy_order.id, 
                sell_order_id: sell_order.id, 
                quantity: qty, 
                price: sell_order.price 
            });

            buy_order.quantity -= qty;
            sell_order.quantity -= qty;

            if buy_order.quantity > 0 { self.buy.insert(0, buy_order); }
            if sell_order.quantity > 0 { self.sell.insert(0, sell_order); }

        }
        trades
    }
}