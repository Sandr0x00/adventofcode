use std::fs;
use std::cmp::Ordering;
use std::collections::HashMap;

fn card_value(card: char, jokers: bool) -> u32 {
    if !jokers {
        return match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    } else {
        return match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        }
    }
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
                // five of a kind
                5 => 7,
                // four of a kind
                4 => 6,
                // three of a kind
                3 => 3,
                // pair
                2 => 1, // => 2 pairs == 2
                // single card
                _ => 0,
            };
        }

        if self.jokers {
            hand_value = match hand_value {
                // five of a kind
                7 => 7, // no space for jokers
                // four of a kind
                6 => 6 + *jokers, // potential five of a kind
                // does not exist
                5 => unreachable!(),
                // full house
                4 => 4, // no space for jokers
                // three of a kind
                3 => match jokers {
                    2 => 7, // five of a kind
                    1 => 6, // four of a kind
                    _ => 3,
                },
                // two pairs
                2 => match jokers {
                    1 => 4, // full house
                    _ => 2,
                },
                // one pair
                1 => match jokers {
                    3 => 7, // five of a kind
                    2 => 6, // four of a kind
                    1 => 3, // three of a kind
                    _ => 1,
                },
                // single card
                _ => match jokers {
                    5 => 7, // five of a kind
                    4 => 7, // five of a kind
                    3 => 6, // four of a kind
                    2 => 3, // three of a kind
                    1 => 1, // one pair
                    _ => 0, // all cards distinct
                },
            }
        }

        return hand_value;
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
            return Ordering::Equal;
        } else {
            return own_hand.cmp(&other_hand);
        }
    }
}

impl PartialEq for Camel {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();

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

    println!("Part One {winnings_one}");
    println!("Part Two {winnings_two}");
}
