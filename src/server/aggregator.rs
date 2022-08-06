use crate::{orderbook_snapshot::OrderbookSnapshot, summary::Summary, limited_collection::LimitedCollection};

pub struct Aggregator<const SIZE: usize> {
    current_spread: f32,
    best_bids: LimitedCollection<SIZE>,
    best_asks: LimitedCollection<SIZE>,
}

impl<const SIZE: usize> Aggregator<SIZE> {
    pub fn new() -> Aggregator<SIZE> {
        Aggregator {
            current_spread: 0.,
            best_bids: LimitedCollection::new(),
            best_asks: LimitedCollection::new()
        }
    }
    pub fn next(&mut self, source: &str, snapshot: OrderbookSnapshot<SIZE>) -> Summary {
        todo!();
    }
}