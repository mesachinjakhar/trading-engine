mod engine;

use engine::order::{Order, Side};

fn main() {
    let mut order = Order::new(1, Side::Buy, 100, 10);

    println!("Initial: {:?}", order);

    order.accept().unwrap();
    println!("After accept: {:?}", order);

    order.fill().unwrap();
    println!("After fill: {:?}", order);

    // this will fail 
    let result = order.cancel();
    println!("Cancel result: {:?}", result);

}
