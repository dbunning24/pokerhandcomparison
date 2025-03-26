#![allow(unused)]
mod parser;
use parser::Parser;

mod evaluate;
use evaluate::Evaluator;

mod hands;

use std::cmp::Ordering;

fn main() -> Result<(), String> {
    println!("welcome to the poker hand comparison machine!");
    let parser = Parser::new();
    let mut eval = Evaluator::new();
    let (hand1, hand1rankscore) = match parser.clone().parse("2C 2D 2H 2S 8H".to_string()) {
        Ok((cards, score)) => (cards, score),
        Err(e) => return Err(e),
    };

    let (hand2, hand2rankscore) = match parser.clone().parse("7D 8C 9S 10C JC".to_string()) {
        Ok((cards, score)) => (cards, score),
        Err(e) => return Err(e),
    };

    let score1 = eval.evaluate(hand1);
    let score2 = eval.evaluate(hand2);

    match score1.cmp(&score2) {
        Ordering::Equal => match hand1rankscore.cmp(&hand2rankscore) {
            Ordering::Greater => println!("hand 1 wins with higher ranking cards!"),
            Ordering::Equal => println!("both hands are equal"),
            Ordering::Less => println!("hand 2 wins with higher ranking cards!"),
        },
        Ordering::Greater => println!("hand 1 wins!"),
        Ordering::Less => println!("hand 2 wins!"),
    }
    Ok(())
}

// generated using chatgpt for testing
#[cfg(test)]
mod tests {
    use super::*;
    use evaluate::Evaluator;
    use parser::Parser;

    fn setup() -> (Parser, Evaluator) {
        let parser = Parser::new();
        let eval = Evaluator::new();
        (parser, eval)
    }

    // Royal Flush Test
    #[test]
    fn test_royal_flush_hand() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("10D JD QD KD AD".to_string()).unwrap();
        let score = eval.evaluate(hand);
        assert_eq!(score, 9); // Assuming '9' is the score for a royal flush
    }

    // Straight Flush Test
    #[test]
    fn test_straight_flush_hand() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("8D 9D 10D JD QD".to_string()).unwrap();
        let score = eval.evaluate(hand);
        assert_eq!(score, 8); // Assuming '8' is the score for a straight flush
    }

    // Straight Test
    #[test]
    fn test_straight_hand() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("8D 9C 10H JD QD".to_string()).unwrap();
        let score = eval.evaluate(hand);
        assert_eq!(score, 4); // Assuming '4' is the score for a straight
    }

    // Flush Test
    #[test]
    fn test_flush_hand() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("2D 4D 6D 8D 10D".to_string()).unwrap();
        let score = eval.evaluate(hand);
        assert_eq!(score, 5); // Assuming '5' is the score for a flush
    }

    // Edge Case: Invalid Hand (Not Enough Cards)
    #[test]
    fn test_invalid_hand_not_enough_cards() {
        let (parser, mut eval) = setup();
        let result = parser.clone().parse("10D JD QD KD".to_string()); // Only 4 cards
        assert!(result.is_err());
    }

    // Edge Case: Invalid Hand (Invalid Card Format)
    #[test]
    fn test_invalid_card_format() {
        let (parser, mut eval) = setup();
        let result = parser.clone().parse("10D JX QD KD AD".to_string()); // 'JX' is not a valid card
        assert!(result.is_err());
    }

    // Edge Case: Invalid Hand (Unsorted Cards)
    #[test]
    fn test_unsorted_flush_hand() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("2D 6D 10D 8D 4D".to_string()).unwrap();
        let score = eval.evaluate(hand);
        assert_eq!(score, 5); // Still a flush
    }

    // Edge Case: High Card (No Special Hand)
    #[test]
    fn test_high_card_hand() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("2D 4C 6H 8S KH".to_string()).unwrap();
        let score = eval.evaluate(hand);
        assert_eq!(score, 0); // Assuming '0' is the score for high card
    }

    // Edge Case: Two Consecutive Same Cards (Invalid)
    #[test]
    fn test_two_consecutive_same_cards() {
        let (parser, mut eval) = setup();
        let result = parser.clone().parse("2D 2D 3H 4S 5C".to_string()); // Same card twice
        assert!(result.is_err());
    }

    // Edge Case: Same Hand in Different Order (Straight)
    #[test]
    fn test_straight_hand_different_order() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("JD 9C 8D QD 10H".to_string()).unwrap(); // Same cards as the straight hand, but in different order
        let score = eval.evaluate(hand);
        assert_eq!(score, 4); // Should still be a straight
    }

    // Edge Case: Mixed Suits but Still a Straight
    #[test]
    fn test_straight_mixed_suits() {
        let (parser, mut eval) = setup();
        let (hand, _) = parser.clone().parse("8D 9H 10C JD QS".to_string()).unwrap(); // A straight with mixed suits
        let score = eval.evaluate(hand);
        assert_eq!(score, 4); // Should still be a straight
    }
}
