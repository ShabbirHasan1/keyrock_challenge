mod aggregator;
mod binance_spot;
mod bitstamp_spot;
mod grpc;
mod level;
mod orderbook_snapshot;
mod summary;

use aggregator::Aggregator;
use orderbook_snapshot::OrderbookSnapshot;

use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let aggregator = Aggregator::new();
    let aggregator = Mutex::new(aggregator);
    let arc_01 = Arc::new(aggregator);
    let arc_02 = arc_01.clone();

    let binance_stream = thread::spawn(move || {
        binance_spot::run_stream(0, arc_01);
    });

    let bitstamp_stream = thread::spawn(move || {
        bitstamp_spot::run_stream(1, arc_02);
    });

    binance_stream.join().unwrap();
    bitstamp_stream.join().unwrap();
}
