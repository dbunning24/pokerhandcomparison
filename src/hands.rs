use std::fmt;

#[derive(Debug)]
pub enum Hands {
    High = 0,
    Pair = 1,
    TwoPair = 2,
    ThreeKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourKind = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}

impl fmt::Display for Hands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hands::High => write!(f, "High Card"),
            Hands::Pair => write!(f, "Pair"),
            Hands::TwoPair => write!(f, "Two Pair"),
            Hands::ThreeKind => write!(f, "Three of a Kind"),
            Hands::Straight => write!(f, "Straight"),
            Hands::Flush => write!(f, "Flush"),
            Hands::FullHouse => write!(f, "Full House"),
            Hands::FourKind => write!(f, "Four of a Kind"),
            Hands::StraightFlush => write!(f, "Straight Flush"),
            Hands::RoyalFlush => write!(f, "Royal Flush"),
        }
    }
}
