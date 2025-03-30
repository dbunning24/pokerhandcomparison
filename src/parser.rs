#[derive(Clone)]
pub struct Parser {
    ranks: [&'static str; 13],
    suits: [char; 4],
}

impl Parser {
    pub fn new() -> Self {
        Self {
            ranks: [
                "2", "3", "4", "5", "6", "7", "8", "9", "10", "j", "q", "k", "a",
            ],
            suits: ['d', 'h', 's', 'c'],
        }
    }
    pub fn parse(self, input: String) -> Result<(Vec<i32>, i8), String> {
        let split: Vec<String> = input
            .split_ascii_whitespace()
            .map(|e| e.to_lowercase().to_string())
            .collect();
        if split.len() != 5 {
            return Err(format!("incorrect amount of cards: {}", split.len()));
        }
        let mut cards: Vec<i32> = Vec::new(); // first 4 bytes = suit, last 13 = rank
        let mut rank_total: i8 = 0;
        for card in split.into_iter() {
            match self.get_hand_pos(card) {
                Ok((r, s)) => {
                    rank_total += r as i8;
                    let c = self.encode_card(r, s);
                    if cards.contains(&c) {
                        return Err("cant enter duplicate cards".into());
                    }
                    cards.push(c);
                }
                Err(e) => return Err(e),
            }
        }
        Ok((cards, rank_total))
    }

    fn get_hand_pos(&self, mut card: String) -> Result<(usize, usize), String> {
        let rank_index = match self
            .ranks
            .iter()
            .position(|e| e.to_string() == &card[0..card.len() - 1])
        {
            Some(e) => e,
            None => {
                dbg!(&card);
                return Err(format!("couldnt validate rank for card {card}"));
            }
        };

        let suit_index = match self
            .suits
            .iter()
            .position(|e| e == &card.chars().nth_back(0).unwrap())
        {
            Some(e) => e,
            None => return Err(format!("couldnt validate suit for card {card}")),
        };
        Ok((rank_index, suit_index))
    }

    fn encode_card(&self, r: usize, s: usize) -> i32 {
        (1 << s) + (1 << 4 + r)
    }
}
