mod orderbook_snapshot;
mod aggregator;
mod binance_spot;
mod bitstamp_spot;
mod summary;
mod limited_collection;
mod level;

use orderbook_snapshot::OrderbookSnapshot;
use aggregator::Aggregator;

use std::{sync::{Arc, Mutex}, thread};

fn main() {

    let aggregator = Aggregator::<10>::new();
    let aggregator = Mutex::new(aggregator);
    let aggregator = Arc::new(aggregator);

    let arc_01 = aggregator.clone();
    let arc_02 = aggregator.clone();

    let binance_stream = thread::spawn(move || {
        binance_spot::run_stream(arc_01);
    });

    let bitstamp_stream = thread::spawn(move || {
        bitstamp_spot::run_stream(arc_02);
    });

    binance_stream.join().unwrap();
    bitstamp_stream.join().unwrap();
}
