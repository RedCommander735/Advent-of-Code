use std::{cmp::Ordering, collections::HashMap, env, fs::read_to_string, iter::zip};

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    hand_type: HandType,
    cards: &'a str,
    bid: u32,
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let order = vec![
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
        ];

        if self.hand_type > other.hand_type {
            return Some(Ordering::Greater);
        }

        if self.hand_type < other.hand_type {
            return Some(Ordering::Less);
        }

        if self.hand_type == other.hand_type {
            let mut ordering = Some(Ordering::Equal);
            for (char_s, char_o) in zip(self.cards.chars(), other.cards.chars()) {
                let index_s = order.iter().position(|&r| r == char_s).unwrap();
                let index_o = order.iter().position(|&r| r == char_o).unwrap();

                if index_s > index_o {
                    ordering = Some(Ordering::Less);
                    break;
                }
                if index_s < index_o {
                    ordering = Some(Ordering::Greater);
                    break;
                }
            }

            return ordering;
        };

        Some(Ordering::Equal)
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type.partial_cmp(&other.hand_type).unwrap()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let mut path = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .canonicalize()
        .unwrap();
    path.push("input");

    let output = match path.to_str() {
        None => panic!("path is not a valid UTF-8 sequence"),
        Some(s) => parse_file(&s),
    };

    println!("\nTotal winnings: {}", output)
}

fn parse_file(path: &str) -> u32 {
    let binding = read_to_string(&path).unwrap();
    let hands = parse_hands(&binding);
    let mut winnings = 0;
    for (index, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (index as u32 + 1)
    }
    winnings
}

fn parse_hands(src: &str) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in src.lines() {
        let l: Vec<&str> = line.split(" ").collect();
        let cards = l[0];
        let bid = l[1];
        let mut card_map: HashMap<char, u8> = HashMap::new();

        for char in cards.chars() {
            if card_map.contains_key(&char) {
                *card_map.get_mut(&char).unwrap() += 1;
            } else {
                card_map.insert(char, 1);
            }
        }

        let len = &card_map.len();

        let hand = {
            let hand_type = match len {
                5 => HandType::HighCard,
                4 => HandType::OnePair,
                3 => {
                    let mut max_val = 0;
                    for (_key, value) in &card_map {
                        if value > &max_val {
                            max_val = *value
                        }
                    }

                    if max_val == 3 {
                        HandType::ThreeOfAKind
                    } else {
                        HandType::TwoPair
                    }
                }
                2 => {
                    let mut max_val = 0;
                    for (_key, value) in &card_map {
                        if value > &max_val {
                            max_val = *value
                        }
                    }

                    if max_val == 4 {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                1 => HandType::FiveOfAKind,
                _ => HandType::HighCard,
            };

            Hand {
                hand_type,
                cards,
                bid: bid.parse().unwrap(),
            }
        };

        hands.append(&mut vec![hand])
    }
    hands.sort();
    hands
}
