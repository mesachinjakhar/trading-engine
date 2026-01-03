use std::collections::HashMap;

use crate::engine::trade::Trade;

use super::order::{Order, Side};
use super::order::OrderState;

pub struct OrderBook {
    buys: HashMap<u64, Order>,
    sells: HashMap<u64, Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buys: HashMap::new(),
            sells: HashMap::new()
        }
    }

    pub fn add_order(&mut self, order: Order) -> Result<(), &'static str> {
        let book = match order.side {
            Side::Buy => &mut self.buys,
            Side::Sell => &mut self.sells
        };

        if book.contains_key(&order.id) {
            return Err("Order with this ID already exists")
        }

        book.insert(order.id, order);
        Ok(())
    }

    pub fn accept_order(&mut self, order_id: u64) -> Result<(), &'static str> {
        let is_buy = self.buys.get_mut(&order_id);
        let is_sell = self.sells.get_mut(&order_id);

        if let Some(order) = is_buy {
            order.accept().unwrap()
        }
        if let Some(order) = is_sell {
            order.accept().unwrap()
        }

        return Err("No orders found with given id")

    }

    // pub fn cancel_order(&mut self, order_id: u64) -> Result<(), &'static str> {
    //     let order = self.orders.get_mut(&order_id).ok_or("order not found")?;
    //     order.cancel()
    // }

    pub fn try_match_once(&mut self) -> Option<Trade> {
        let buy = self.buys.values_mut().next()?;
        let sell = self.sells.values_mut().next()?;

        if buy.price >= sell.price {
            let buy_result  = buy.fill();
            sell.fill().ok()?;

            let trade = Trade {
                buy_order_id: buy.id,
                sell_order_id: sell.id, 
                price: sell.price,
                quantity: buy.quantity.min(sell.quantity)
            };

            return Some(trade);
        }

        None
    }
}