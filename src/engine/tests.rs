use super::book::OrderBook;
use super::order::{Order, Side};

#[test]
fn high_buy_price_wins() {
    let mut book = OrderBook::new();

    // lower price buy
    let mut buy1 = Order::new(1, Side::Buy, 100, 10);

    // higher price buy
    let mut buy2 = Order::new(2, Side::Buy, 110, 10); 

    // sell order
    let mut sell1 = (Order::new(3, Side::Sell, 90, 5));

    buy1.accept().unwrap();
    buy2.accept().unwrap();
    sell1.accept().unwrap();

    book.add_order(buy1).unwrap();
    book.add_order(buy2).unwrap();
    book.add_order(sell1).unwrap();



    let trade = book.try_match_once().expect("Trade should happend");
    assert_eq!(trade.buy_order_id, 2);
    assert_eq!(trade.price, 90);
    assert_eq!(trade.quantity, 5);

}

#[test]

fn fifo_is_respected_for_same_price() {
    let mut book = OrderBook::new();

    // two sell orders, same price
    let mut sell1 = Order::new(1, Side::Sell, 60, 5);
    let mut sell2 = Order::new(2, Side::Sell, 60, 5);

    sell1.accept().unwrap();
    sell2.accept().unwrap();


    sell1.seq = 1;
    sell2.seq = 2;

    book.add_order(sell1).unwrap();
    book.add_order(sell2).unwrap();

    // Buyer
    let mut buy1 = Order::new(3, Side::Buy, 100, 5);

    buy1.accept().unwrap();

    book.add_order(buy1).unwrap();



    let trade = book.try_match_once().expect("trade should happen");

    assert_eq!(trade.sell_order_id, 1);
}


#[test]

fn partial_fill_leaves_reaming_order() {
    let mut book = OrderBook::new();

    // big buy
    let mut buy = Order::new(1, Side::Buy, 100, 10);

    // small sell
    let mut sell1 = Order::new(2, Side::Sell, 90, 4);

    buy.accept().unwrap();
    sell1.accept().unwrap();

    book.add_order(buy).unwrap();
    book.add_order(sell1).unwrap();

    let trade = book.try_match_once().expect("Trade should happen");
    assert_eq!(trade.quantity, 4);

    let trade2 = book.try_match_once();
    assert!(trade2.is_none());

}