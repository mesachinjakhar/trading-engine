mod engine;

use engine::order::{Order, Side};

use crate::engine::book::OrderBook;

fn main() {

    let mut book = OrderBook::new();

    let mut buy = Order::new(1, Side::Buy, 100, 10);
    let mut  sell = Order::new(2, Side::Sell, 90, 5);

    buy.accept().unwrap();
    sell.accept().unwrap();

    book.add_order(buy).unwrap();
    book.add_order(sell).unwrap();

    let trade = book.try_match_once();


    let trade2 = book.try_match_once();

    println!("trade result: {:?}", trade);
    println!("trade result: {:?}", trade2);


    // println!("after trade: {:?} {:?}", buy, sell)


}
