#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderState {
    Created,
    Accepted,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
    pub state: OrderState
}

impl Order {
    pub fn new(id: u64, side: Side, price: u64, quantity: u64) -> Self {
        Self {
            id, 
            side, 
            price,
            quantity,
            state: OrderState::Created,
        }
    }
}