use crate::{level::Level, orderbook_snapshot::OrderbookSnapshot, summary::Summary};
use init_with::InitWith;

pub struct Aggregator {
    best_bids_01: Option<[Level; 10]>,
    best_bids_02: Option<[Level; 10]>,
    best_asks_01: Option<[Level; 10]>,
    best_asks_02: Option<[Level; 10]>,
}

impl Aggregator {
    pub fn new() -> Aggregator {
        Aggregator {
            best_bids_01: None,
            best_bids_02: None,
            best_asks_01: None,
            best_asks_02: None,
        }
    }
    pub fn process(&mut self, source_id: usize, snapshot: OrderbookSnapshot<10>) -> Summary<10> {
        match source_id {
            0 => {
                self.best_bids_01 = Some(snapshot.bids);
                self.best_asks_01 = Some(snapshot.asks);
            }
            1 => {
                self.best_bids_02 = Some(snapshot.bids);
                self.best_asks_02 = Some(snapshot.asks);
            }
            _ => panic!("The aggregator currently only supports two market streams"),
        }

        let mut merged_best_bids: [Option<Level>; 10] = [None; 10];
        let mut merged_best_asks: [Option<Level>; 10] = [None; 10];

        if self.best_bids_01.is_some() && self.best_bids_02.is_some() {
            Aggregator::merge(
                &mut merged_best_bids,
                &self.best_bids_01.unwrap(),
                &self.best_bids_02.unwrap(),
                0,
                0,
                0,
                false,
            );
            Aggregator::merge(
                &mut merged_best_asks,
                &self.best_asks_01.unwrap(),
                &self.best_asks_02.unwrap(),
                0,
                0,
                0,
                true,
            );

            let bids_normalized: [Level; 10] = Aggregator::normalize(&merged_best_bids);
            let asks_normalized: [Level; 10] = Aggregator::normalize(&merged_best_asks);

            return Summary {
                spread: asks_normalized[0].price - bids_normalized[0].price,
                bids: bids_normalized,
                asks: asks_normalized,
            };
        }

        if self.best_bids_01.is_some() {
            Summary {
                spread: self.best_asks_01.unwrap()[0].price - self.best_bids_01.unwrap()[0].price,
                bids: self.best_bids_01.unwrap(),
                asks: self.best_asks_01.unwrap(),
            }
        } else {
            Summary {
                spread: self.best_asks_02.unwrap()[0].price - self.best_bids_02.unwrap()[0].price,
                bids: self.best_bids_02.unwrap(),
                asks: self.best_asks_02.unwrap(),
            }
        }
    }

    fn merge(
        merged: &mut [Option<Level>; 10],
        levels_01: &[Level; 10],
        levels_02: &[Level; 10],
        merged_index: usize,
        index_01: usize,
        index_02: usize,
        side: bool,
    ) {
        if merged_index == 10 {
            return;
        }

        let mut new_index_01 = index_01;
        let mut new_index_02 = index_02;

        if side {
            // asks
            let level_01 = &levels_01[index_01];
            let level_02 = &levels_02[index_02];

            if level_01.price > level_02.price {
                merged[merged_index] = Some(*level_02);
                new_index_02 += 1;
            } else {
                merged[merged_index] = Some(*level_01);
                new_index_01 += 1;
            }
        } else {
            // bids
            let level_01 = &levels_01[9 - index_01];
            let level_02 = &levels_02[9 - index_02];

            if level_01.price > level_02.price {
                merged[merged_index] = Some(*level_01);
                new_index_01 += 1;
            } else {
                merged[merged_index] = Some(*level_02);
                new_index_02 += 1;
            }
        }

        Aggregator::merge(
            merged,
            levels_01,
            levels_02,
            merged_index + 1,
            new_index_01,
            new_index_02,
            side,
        )
    }

    fn normalize(original: &[Option<Level>; 10]) -> [Level; 10] {
        <[Level; 10]>::init_with_indices(|i| original[i].unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Aggregator;
    use crate::level::Level;
    use init_with::InitWith;

    #[test]
    fn should_merge_bids() {
        // Arrange
        let mut merged: [Option<Level>; 10] = [None; 10];
        let levels_01 = <[Level; 10]>::init_with_indices(|i| Level {
            price: 10. + i as f64,
            amount: 13.,
            exchange: [' '; 10],
        });
        let levels_02 = <[Level; 10]>::init_with_indices(|i| Level {
            price: 6. + 2. * i as f64,
            amount: 37.,
            exchange: [' '; 10],
        });

        // Act
        Aggregator::merge(&mut merged, &levels_01, &levels_02, 0, 0, 0, false);

        // Assert
        assert!(merged[0].unwrap().amount == 37. && merged[0].unwrap().price == 24.);
        assert!(merged[1].unwrap().amount == 37. && merged[1].unwrap().price == 22.);
        assert!(merged[2].unwrap().amount == 37. && merged[2].unwrap().price == 20.);
        assert!(merged[3].unwrap().amount == 13. && merged[3].unwrap().price == 19.);
        assert!(merged[4].unwrap().amount == 37. && merged[4].unwrap().price == 18.);
        assert!(merged[5].unwrap().amount == 13. && merged[5].unwrap().price == 18.);
        assert!(merged[6].unwrap().amount == 13. && merged[6].unwrap().price == 17.);
        assert!(merged[7].unwrap().amount == 37. && merged[7].unwrap().price == 16.);
        assert!(merged[8].unwrap().amount == 13. && merged[8].unwrap().price == 16.);
        assert!(merged[9].unwrap().amount == 13. && merged[9].unwrap().price == 15.);
    }

    #[test]
    fn should_merge_asks() {
        // Arrange
        let mut merged: [Option<Level>; 10] = [None; 10];
        let levels_01 = <[Level; 10]>::init_with_indices(|i| Level {
            price: 10. + i as f64,
            amount: 13.,
            exchange: [' '; 10],
        });
        let levels_02 = <[Level; 10]>::init_with_indices(|i| Level {
            price: 6. + 2. * i as f64,
            amount: 37.,
            exchange: [' '; 10],
        });

        // Act
        Aggregator::merge(&mut merged, &levels_01, &levels_02, 0, 0, 0, true);

        // Assert
        assert!(merged[0].unwrap().amount == 37. && merged[0].unwrap().price == 6.);
        assert!(merged[1].unwrap().amount == 37. && merged[1].unwrap().price == 8.);
        assert!(merged[2].unwrap().amount == 13. && merged[2].unwrap().price == 10.);
        assert!(merged[3].unwrap().amount == 37. && merged[3].unwrap().price == 10.);
        assert!(merged[4].unwrap().amount == 13. && merged[4].unwrap().price == 11.);
        assert!(merged[5].unwrap().amount == 13. && merged[5].unwrap().price == 12.);
        assert!(merged[6].unwrap().amount == 37. && merged[6].unwrap().price == 12.);
        assert!(merged[7].unwrap().amount == 13. && merged[7].unwrap().price == 13.);
        assert!(merged[8].unwrap().amount == 13. && merged[8].unwrap().price == 14.);
        assert!(merged[9].unwrap().amount == 37. && merged[9].unwrap().price == 14.);
    }
}
