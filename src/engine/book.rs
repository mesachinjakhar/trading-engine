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

        let buy_id = self.best_buy_id()?;
        let sell_id = self.best_sell_id()?;

        let buy = self.buys.get_mut(&buy_id)?;
        let sell = self.sells.get_mut(&sell_id)?;

        if buy.price < sell.price {
            return None;
        }

        let trade_qty = buy.remaining.min(sell.remaining);

        buy.apply_fill(trade_qty).ok()?;
        sell.apply_fill(trade_qty).ok()?;

            let trade = Trade {
                buy_order_id: buy.id,
                sell_order_id: sell.id, 
                price: sell.price,
                quantity: trade_qty
            };


            // cleanup only filled orders
            if buy.state == OrderState::Filled { 
                self.buys.remove(&buy_id);
            }
            if sell.state == OrderState::Filled {
                self.sells.remove(&sell_id);
            }

            return Some(trade);
        
    }


    fn best_buy_id(&self) -> Option<u64> {
        self.buys
        .iter()
        .max_by_key(|(_, order)| order.price)
        .map(|(id,_)| *id)
    }

    fn best_sell_id(&self) -> Option<u64> {
        self.sells
        .iter()
        .min_by_key(|(_, order)| order.price)
        .map(|(id, _)| *id)
    }




}