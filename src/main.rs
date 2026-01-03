mod engine;

use engine::order::{Order, Side};

use crate::engine::book::OrderBook;

fn main() {

    let mut book = OrderBook::new();

    let order1 = Order::new(1, Side::Buy, 100, 10);
    let order2 = Order::new(2, Side::Sell, 105, 5);

    book.add_order(order1).unwrap();
    book.add_order(order2).unwrap();

    book.accept_order(1).unwrap();
    book.cancel_order(2).unwrap();

    println!("Orderbook test completed");

}
