use crate::level::Level;

pub struct LimitedCollection<const SIZE: usize> {
    collection: [Option<Level>; SIZE],
    index: usize
}

impl<const SIZE: usize> LimitedCollection<SIZE> {
    pub fn new() -> LimitedCollection<SIZE> {
        LimitedCollection {
            collection: [None; SIZE],
            index: 0,
        }
    }
    pub fn next(&self, level: Level) {
        todo!();
    }
}

impl<const SIZE: usize> Iterator for LimitedCollection<SIZE> {
    type Item = Level;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}