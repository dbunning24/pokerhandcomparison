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
