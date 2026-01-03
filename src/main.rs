mod engine;

use engine::order::{Order, Side};

fn main() {
    let order = Order::new(1, Side::Buy, 100, 10);

    println!("{:?}", order);
}
