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
        println!("hand: {:?} > {:?}", self.cards, hand);
        hand as i8
    }

    fn determine_hand(&self) -> Hands {
        // check for straights and flushes
        let mut hand = 0;
        let mut suit_bits = 0;
        for card in self.cards.iter() {
            hand ^= card >> 4; // xor to set same-rank cards to 0, shift card 4 bits right to remove suit
            suit_bits |= card & 0xF; // extract lower 4 bits to get suit
        }
        match (
            (hand >> hand.trailing_zeros()) == 0x1F, // remove all trailing zeros and check if all values are set to 1
            suit_bits.count_ones() == 1,             // check if there's only one suit bit set to 1
        ) {
            (true, true) if hand.trailing_zeros() == 8 => return Hands::RoyalFlush,
            (true, true) => return Hands::StraightFlush,
            (true, false) => return Hands::Straight,
            (false, true) => return Hands::Flush,
            (false, false) => {
                self.find_pairs();
                return Hands::High;
            }
        }
    }

    fn find_pairs(&self) -> Hands {
        Hands::High
    }
}
