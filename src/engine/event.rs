use super::order::Order;

#[derive(Debug)]
pub enum EngineEvent {
    NewOrder(Order),
    CancelOrder(u64),
    Shutdown,
}
