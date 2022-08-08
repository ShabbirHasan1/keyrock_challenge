use crate::level::Level;

pub struct OrderbookSnapshot<const DEPTH: usize> {
    pub bids: [Level; DEPTH],
    pub asks: [Level; DEPTH],
}
