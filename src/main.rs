mod engine;

use engine::order::{Order, Side};

use crate::engine::book::OrderBook;

fn main() {

    let mut book = OrderBook::new();

    let mut buy: Order = Order::new(1, Side::Buy, 100, 10);
    let mut  buy2 = Order::new(2, Side::Buy, 105, 10);
    let mut  sell = Order::new(2, Side::Sell, 95, 5);

    buy.accept().unwrap();
    buy2.accept().unwrap();
    sell.accept().unwrap();

    book.add_order(buy).unwrap();
    book.add_order(buy2).unwrap();
    book.add_order(sell).unwrap();

    let trade = book.try_match_once();

    println!("trade result: {:?}", trade);

    // println!("after trade: {:?} {:?}", buy, sell)


}
