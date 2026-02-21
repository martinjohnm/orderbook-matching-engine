

use std::collections::{BTreeMap, VecDeque};

use crate::models::{Order};

#[derive(Debug)]
pub struct OrderBookBtreeMap {
    bids : BTreeMap<u64, VecDeque<Order>>, // descending order for bids
    asks : BTreeMap<u64, VecDeque<Order>>  // ascending order for asks
}

impl OrderBookBtreeMap {
    pub fn new() -> Self {
        Self { 
            bids: BTreeMap::new() ,
            asks: BTreeMap::new() 
        }
    }

    pub fn insert_order(&mut self, mut order: Order, is_buy: bool) {
        if is_buy {
            // Match with asks
            while order.quantity > 0 {
                if let Some((&ask_price, ask_orders)) = self.asks.iter_mut().next() {
                    if ask_price > order.price {
                        break; // no match
                    }

                    // Take first order at this price
                    let mut existing_order = ask_orders.pop_front().unwrap();
                    let traded_qty = order.quantity.min(existing_order.quantity);

                    // Log trade
                    

                    // Reduce quantities
                    order.quantity -= traded_qty;
                    existing_order.quantity -= traded_qty;

                    if existing_order.quantity > 0 {
                        ask_orders.push_front(existing_order);
                    }

                    if ask_orders.is_empty() {
                        self.asks.remove(&ask_price);
                    }
                } else {
                    break; // no more asks
                }
            }

            // Remaining goes to book
            if order.quantity > 0 {
                self.bids.entry(order.price).or_insert_with(VecDeque::new).push_back(order);
            }
        } else {
            // Match with bids
            while order.quantity > 0 {
                if let Some((&bid_price, bid_orders)) = self.bids.iter_mut().rev().next() {
                    if bid_price < order.price {
                        break; // no match
                    }

                    let mut existing_order = bid_orders.pop_front().unwrap();
                    let traded_qty = order.quantity.min(existing_order.quantity);

                    

                    order.quantity -= traded_qty;
                    existing_order.quantity -= traded_qty;

                    if existing_order.quantity > 0 {
                        bid_orders.push_front(existing_order);
                    }

                    if bid_orders.is_empty() {
                        self.bids.remove(&bid_price);
                    }
                } else {
                    break; // no more bids
                }
            }

            if order.quantity > 0 {
                self.asks.entry(order.price).or_insert_with(VecDeque::new).push_back(order);
            }
        }
    }

    // Get best bid
    fn _best_bid(&self) -> Option<(&u64, &VecDeque<Order>)> {
        self.bids.iter().rev().next() // highest bid price
    }

    // Get best ask
    fn _best_ask(&self) -> Option<(&u64, &VecDeque<Order>)> {
        self.asks.iter().next() // lowest ask price
    }

}