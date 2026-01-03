use std::collections::HashMap;

use super::order::Order;
use super::order::OrderState;

pub struct OrderBook {
    orders: HashMap<u64, Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) -> Result<(), &'static str> {
        if self.orders.contains_key(&order.id) {
            return Err("Order with this ID already exists")
        }

        self.orders.insert(order.id, order);
        Ok(())
    }

    pub fn accept_order(&mut self, order_id: u64) -> Result<(), &'static str> {
        let order = self.orders.get_mut(&order_id).ok_or("Order not found")?;

        order.accept()
    }

    pub fn cancel_order(&mut self, order_id: u64) -> Result<(), &'static str> {
        let order = self.orders.get_mut(&order_id).ok_or("order not found")?;
        order.cancel()
    }
}