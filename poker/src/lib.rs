use std::cmp::Ordering;
// Inspired from https://exercism.io/tracks/rust/exercises/poker/solutions/5ec8d482e2674642a8e42601724dabda

#[derive(Debug)]
struct Card {
    rank: u8,
    suit: u8,
}

impl Card {
    fn new(c: &str) -> Card {
        let rank: char = c.chars().nth(0).unwrap();
        let suit: char = c.chars().nth(c.len() - 1).unwrap();

        let rank: u8 = match rank {
            '2'..='9' => rank.to_digit(10).unwrap(),
            '1' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!()
        } as u8;

        let suit: u8 = match suit {
            'H' => 1,
            'D' => 2,
            'S' => 3,
            'C' => 4,
            _ => panic!()
        };

        Card { rank, suit }
    }
}

#[derive(Debug)]
struct Hand<'a> {
    cards: Vec<Card>,
    s: &'a str,
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.values().partial_cmp(&other.values())
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.values() == other.values()
    }
}

impl<'a> Hand<'a> {
    fn new(s: &'a str) -> Hand {
        let mut cards = Vec::with_capacity(5);
        for card in s.split_whitespace() {
            cards.push(Card::new(card))
        }

        Hand { cards, s }
    }

    fn is_flush(&self) -> bool {
        let first = self.cards.first().unwrap();
        for card in self.cards.iter() {
            if first.suit != card.suit {
                return false;
            }
        }

        true
    }

    fn is_straight(&self) -> u32 {
        let mut ranks = [0; 14];

        for card in self.cards.iter() {
            ranks[card.rank as usize - 1] = 1;

            // Ace can also be used as lower rank in straight.
            if card.rank == 14 {
                ranks[0] = 1;
            }
        }

        let ranks: Vec<(usize, &i32)> = ranks.iter().enumerate().collect();
        let value: u32 = ranks.windows(5).map(|rank_win| {
            if rank_win.iter().all(|rank| rank.1 == &1) {
                return rank_win[4].0 as u32 + 1;
            }

            return 0;
        }).max().unwrap();

        value
    }

    fn values(&self) -> [u32; 8] {
        let mut hv = [0; 8];

        let mut card_rank_count = [0; 14];
        for card in self.cards.iter() {
            card_rank_count[(card.rank - 1) as usize] += 1;
        }

        for (i, count) in card_rank_count.iter().enumerate().rev() {
            match count {
                4 => hv[1] = i as u32,
                3 => hv[5] = i as u32,
                2 => hv[6] = (hv[6] * 100) + i as u32,
                1 => hv[7] = (hv[7] * 100) + i as u32,
                0 => {}
                _ => panic!()
            }

            hv[2] = (hv[5] > 0 && hv[6] > 0) as u32;
            hv[3] = self.is_flush() as u32;
            hv[4] = self.is_straight();
            hv[0] = (hv[3] > 0 && hv[4] > 0) as u32;
        }

        hv
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut result: Vec<&str> = vec![];
    let mut hands: Vec<Hand> = hands.iter().map(|h| {
        Hand::new(h)
    }).collect();

    hands.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let first_hand = &hands[0];

    for hand in hands.iter() {
        if *hand == *first_hand {
            result.push(hand.s)
        }
    }

    for hand in hands.iter() {
        println!("{:?}", hand.values());
    }

    Some(result)
}