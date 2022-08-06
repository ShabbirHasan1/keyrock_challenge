use crate::level::Level;

pub struct Summary {
    pub spread: f64,
    pub bids: [Level; 10],
    pub asks: [Level; 10]
}