mod engine;

use engine::order::{Order, Side};

use crate::engine::book::OrderBook;

fn main() {

    let mut book = OrderBook::new();

    let mut buy_1: Order = Order::new(1, Side::Buy, 60, 5, 3);
    let mut buy_2: Order = Order::new(4, Side::Buy, 70, 5, 4);

    let mut  sell_1 = Order::new(2, Side::Sell, 50, 5, 1);
    let mut  sell_2 = Order::new(3, Side::Sell, 50, 5, 2);
    let mut  sell_3 = Order::new(5, Side::Sell, 10, 5, 5);

    buy_1.accept().unwrap();
    buy_2.accept().unwrap();
    sell_1.accept().unwrap();
    sell_2.accept().unwrap();
    sell_3.accept().unwrap();

    book.add_order(sell_1).unwrap();
    book.add_order(sell_2).unwrap();
    book.add_order(sell_3).unwrap();
    book.add_order(buy_1).unwrap();
    book.add_order(buy_2).unwrap();


    let trade = book.try_match_once();

    println!("trade result: {:?}", trade);

}
