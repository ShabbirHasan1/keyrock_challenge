use crate::level::Level;

pub struct Summary<const SIZE: usize> {
    pub spread: f64,
    pub bids: [Level; SIZE],
    pub asks: [Level; SIZE],
}
