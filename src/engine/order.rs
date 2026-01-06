#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderState {
    Created,
    Accepted,
    Filled,
    PartiallyFilled,
    Cancelled,
    Rejected,

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
    pub state: OrderState,
    pub remaining: u64,
    pub seq: u64,
}

impl Order {
    pub fn new(id: u64, side: Side, price: u64, quantity: u64, seq: u64) -> Self {
        Self {
            id, 
            side, 
            price,
            quantity,
            state: OrderState::Created,
            remaining: quantity,
            seq
        }
    }

    pub fn accept(&mut self) -> Result<(), &'static str> {
        if self.state != OrderState::Created {
            return  Err("Only Created orders can be accepted");
        }

        self.state = OrderState::Accepted;
        Ok(())
    }

    pub fn fill(&mut self) -> Result<(), &'static str> {
        if self.state != OrderState::Accepted {
            return Err("Only Accepted orders can be filled");
        }

        self.state = OrderState::Filled;
        Ok(())
    }

    pub fn apply_fill(&mut self, qty: u64) -> Result<(), &'static str> {
        if self.state != OrderState::Accepted && self.state != OrderState::PartiallyFilled {
            return Err("Order not fillable in current state");
        }

        if qty > self.remaining {
        return Err("Fill quantiy exceeds remaining");
    }

    self.remaining -= qty;

    if self.remaining == 0 {
        self.state = OrderState::Filled;
    } else {
        self.state = OrderState::PartiallyFilled;
    }

    Ok(())

    }


    pub fn cancel(&mut self) -> Result<(), &'static str> {
        match self.state {
            OrderState::Created | OrderState::Accepted => {
                self.state = OrderState::Cancelled;
                Ok(())
            }
            _ => Err("Order cannot be cancelled in the current state")
        }
    }

    pub fn reject(&mut self) -> Result<(), &'static str> {
        if self.state != OrderState::Created {
            return Err("Only created orders can be rejected");
        }

        self.state = OrderState::Rejected;
        Ok(())
    }

}