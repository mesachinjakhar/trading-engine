use tokio::signal;
use tokio::sync::mpsc;
use tracing::info;

mod engine;

use engine::order::{Order, Side};

use crate::engine::{book::OrderBook, engine::Engine, event::EngineEvent};

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(100);

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("Trading engine starting");

    // spawn engine
    let engine = Engine::new();
    let engine_handle = tokio::spawn(engine.run(rx));

    let shutdown_tx = tx.clone();

    tokio::spawn(async move {
        signal::ctrl_c()
            .await
            .expect("failed to listen for ctrl + c");
        info!("CTRL+C recieved, shutting down engine...");
        let _ = shutdown_tx.send(EngineEvent::Shutdown).await;
    });

    tx.send(EngineEvent::NewOrder(Order::new(1, Side::Buy, 100, 10)))
        .await
        .unwrap();

    tx.send(EngineEvent::NewOrder(Order::new(2, Side::Sell, 90, 5)))
        .await
        .unwrap();

    // tx.send(EngineEvent::Shutdown).await.unwrap();

    engine_handle.await.unwrap();
}
