use std::cmp::Ordering;
use std::collections::HashMap;

const DAY: u8 = 7;

fn card_value(card: char, jokers: bool) -> u32 {
    if !jokers {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    } else {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }
}

enum Hand {
    Five = 7,
    Four = 6,
    Three = 3,
    Pair = 1,
    HighCard = 0,

    FullHouse = 4, // Three + Pair
    TwoPair = 2, // Pair + Pair
}

#[derive(Debug, Eq)]
struct Camel {
    hand: Vec<char>,
    bid: usize,
    jokers: bool,
}

impl Camel {
    fn hand_type(&self) -> usize {
        let mut strength = HashMap::new();

        strength.entry(&'J').or_insert(0);
        for c in &self.hand {
            strength.entry(c).and_modify(|m| *m += 1).or_insert(1);
        }

        let jokers = &strength.get(&'J').unwrap().clone();

        let mut hand_value = 0;
        // 7 five of a kind
        // 6 four of a kind
        // 4 full house
        // 3 three of a kind
        // 2 two pair
        // 1 one pair
        // 0 high card
        for (card, count) in strength {
            if self.jokers && card == &'J' {
                // skip jokers
                continue;
            }
            hand_value += match count {
                5 => Hand::Five,
                4 => Hand::Four,
                3 => Hand::Three, // Three + Pair == FullHouse
                2 => Hand::Pair, // => Pair + Pair == TwoPair
                _ => Hand::HighCard,
            } as usize;
        }

        let mut hand = match hand_value {
            7 => Hand::Five,
            6 => Hand::Four,
            5 => unreachable!(),
            4 => Hand::FullHouse,
            3 => Hand::Three,
            2 => Hand::TwoPair,
            1 => Hand::Pair,
            _ => Hand::HighCard,
        };

        if self.jokers {
            hand = match hand {
                // five of a kind
                Hand::Five => Hand::Five, // no space for jokers
                // four of a kind
                Hand::Four => match jokers {
                    1 => Hand::Five, // five of a kind
                    _ => Hand::Four,
                },
                // full house
                Hand::FullHouse => Hand::FullHouse, // no space for jokers
                // three of a kind
                Hand::Three => match jokers {
                    2 => Hand::Five, // five of a kind
                    1 => Hand::Four, // four of a kind
                    _ => Hand::Three,
                },
                // two pairs
                Hand::TwoPair => match jokers {
                    1 => Hand::FullHouse, // full house
                    _ => Hand::TwoPair,
                },
                // one pair
                Hand::Pair => match jokers {
                    3 => Hand::Five, // five of a kind
                    2 => Hand::Four, // four of a kind
                    1 => Hand::Three, // three of a kind
                    _ => Hand::Pair,
                },
                // single card
                _ => match jokers {
                    5 => Hand::Five, // five of a kind
                    4 => Hand::Five, // five of a kind
                    3 => Hand::Four, // four of a kind
                    2 => Hand::Three, // three of a kind
                    1 => Hand::Pair, // one pair
                    _ => Hand::HighCard, // all cards distinct
                },
            };
        }

        hand as usize
    }
}

impl PartialOrd for Camel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Camel {
    fn cmp(&self, other: &Self) -> Ordering {
        let own_hand = self.hand_type();
        let other_hand = other.hand_type();

        if own_hand == other_hand {
            // hand equally strong, find better card
            for i in 0..5 {
                if card_value(self.hand[i], self.jokers) < card_value(other.hand[i], self.jokers) {
                    return Ordering::Less;
                }
                if card_value(self.hand[i], self.jokers) > card_value(other.hand[i], self.jokers) {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        } else {
            own_hand.cmp(&other_hand)
        }
    }
}

impl PartialEq for Camel {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

#[allow(dead_code)]
pub fn solve() {
    let input = aoc::input(DAY);

    let mut hands_one = Vec::new();
    let mut hands_two = Vec::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        let camel_one = Camel {
            hand: parts[0].chars().collect(),
            bid: parts[1].parse::<usize>().unwrap(),
            jokers: false,
        };
        hands_one.push(camel_one);
        let camel_two = Camel {
            hand: parts[0].chars().collect(),
            bid: parts[1].parse::<usize>().unwrap(),
            jokers: true,
        };
        hands_two.push(camel_two);
    }

    hands_one.sort();
    hands_two.sort();

    let mut winnings_one = 0;
    for (i, camel) in hands_one.iter().enumerate() {
        winnings_one += camel.bid * (i + 1);
    }
    let mut winnings_two = 0;
    for (i, camel) in hands_two.iter().enumerate() {
        winnings_two += camel.bid * (i + 1);
    }

    aoc::print_solution(DAY, &[
        winnings_one,
        winnings_two,
    ])
}
