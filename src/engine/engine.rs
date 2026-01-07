use super::book::OrderBook;
use super::event::EngineEvent;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub struct Engine {
    book: OrderBook,
    next_seq: u64,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            book: OrderBook::new(),
            next_seq: 1,
        }
    }

    pub fn handle_event(&mut self, event: EngineEvent) {
        match event {
            EngineEvent::NewOrder(mut order) => {
                order.seq = self.next_seq;
                self.next_seq += 1;

                let _ = self.book.add_order(order);
                let _ = self.book.try_match_once();
            }
            EngineEvent::CancelOrder(order_id) => {
                let _ = self.book.cancel_order(order_id);
            }
            EngineEvent::Shutdown => {
                println!("Engine shutting down");
            }
        }
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<EngineEvent>) {
        info!("Engine started");
        while let Some(event) = rx.recv().await {
            match event {
                EngineEvent::NewOrder(order) => {
                    info!(order_id = order.id, side = ?order.side, price = order.price, qty = order.quantity, "new order recieved");

                    let _ = self.book.add_order(order);

                    while let Some(trade) = self.book.try_match_once() {
                        info!(
                            buy = trade.buy_order_id,
                            sell = trade.sell_order_id,
                            price = trade.price,
                            qty = trade.quantity,
                            "Trade executed"
                        );
                    }
                }

                EngineEvent::CancelOrder(order_id) => {
                    warn!(order_id, "cancel order requested");
                    let _ = self.book.cancel_order(order_id);
                }
                EngineEvent::Shutdown => {
                    info!("Engine shutdow signal recieved");
                    break;
                }
            }
        }

        info!("Engine stopeed");
    }
}
