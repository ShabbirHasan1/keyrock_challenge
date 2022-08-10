use keyrock_challenge_proto::orderbook::Level;

pub struct OrderbookSnapshot<const DEPTH: usize> {
    pub bids: [Level; DEPTH],
    pub asks: [Level; DEPTH],
}
