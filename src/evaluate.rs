use crate::hands::Hands;

pub struct Evaluator {
    cards: Vec<i32>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }
    pub fn evaluate(&mut self, cards: Vec<i32>) -> i8 {
        self.cards = cards;
        let hand = self.determine_hand();
        //println!("hand: {:?} > {:?}", self.cards, hand);
        println!("you drew a {}!", hand);
        hand as i8
    }

    fn determine_hand(&self) -> Hands {
        // check for straights and flushes
        let mut rank_bits = 0;
        let mut suit_bits = 0;
        for card in self.cards.iter() {
            rank_bits ^= card >> 4; // xor to set same-rank cards to 0, shift card 4 bits right to remove suit
            suit_bits |= card & 0xF; // extract lower 4 bits to get suit
        }
        match (
            ((rank_bits >> rank_bits.trailing_zeros()) == 0x1F || rank_bits == 0x100F), // remove all trailing zeros and check if all values are set to 1 / check for ace straight
            suit_bits.count_ones() == 1, // check if there's only one suit bit set to 1
        ) {
            (true, true) if rank_bits.trailing_zeros() == 8 => Hands::RoyalFlush,
            (true, true) => Hands::StraightFlush,
            (true, false) => Hands::Straight,
            (false, true) => Hands::Flush,
            (false, false) => self.find_pairs(),
        }
    }

    fn find_pairs(&self) -> Hands {
        // collect all rank bits into vec
        let mut ranks: Vec<i32> = self.cards.iter().map(|card| card >> 4).collect();
        ranks.sort();

        // go through each rank and get matching pairs

        let mut paired_ranks: Vec<Vec<&i32>> = ranks
            .iter()
            .map(|rank| ranks.iter().filter(|r| r == &rank).collect())
            .collect();

        paired_ranks.dedup();

        // filter all 1s out
        let mut filtered_ranks: Vec<i32> = paired_ranks
            .into_iter()
            .filter(|rank| rank.len() > 1)
            .map(|rank| rank.len() as i32)
            .collect();

        filtered_ranks.sort();
        match (
            filtered_ranks.iter().nth(0).unwrap_or(&0),
            filtered_ranks.iter().nth(1).unwrap_or(&0),
        ) {
            (4, 0) => Hands::FourKind,
            (2, 3) => Hands::FullHouse,
            (3, 0) => Hands::ThreeKind,
            (2, 2) => Hands::TwoPair,
            (2, 0) => Hands::Pair,
            _ => Hands::High,
        }
    }
}
