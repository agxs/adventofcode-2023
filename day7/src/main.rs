use std::{fs::File, io::{self, Read}, collections::HashMap, cmp::Ordering};
use logos::{Logos, Lexer};
use thiserror::Error;

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: i32,
}

static HIGH_CARD: i32 = 1;
static PAIR: i32 = 2;
static TWO_PAIR: i32 = 3;
static THREE: i32 = 4;
static FULL_HOUSE: i32 = 5;
static FOUR: i32 = 6;
static FIVE: i32 = 7;

#[derive(Debug, Error)]
#[error("Invalid card")]
struct InvalidCard;

fn hand_rank(cards: [u8; 5]) -> i32 {
    let mut card_count: HashMap<u8, i32> = HashMap::new();
    for card in cards {
        if card == 1 {
            continue;
        }
        *card_count.entry(card).or_insert(0) += 1;
    }
    // all 1s case
    if card_count.len() == 0 {
        return FIVE;
    }
    let mut card_values: Vec<i32> = card_count.values().cloned().collect();
    card_values.sort();
    card_values.reverse();
    let jokers: i32 = cards.iter().map(|c| match *c {
        1 => 1,
        _ => 0,
    }).sum();

    card_values[0] += jokers;
    // println!("Card {:?} is {:?}", cards, card_values);

    return match card_values[..] {
        [_x] => FIVE,
        [x, _y] => {
            if x == 4 {
                return FOUR;
            }
            FULL_HOUSE
        },
        [x, _y, _z] => {
            if x == 3 {
                return THREE;
            }
            TWO_PAIR
        }
        [_x, _y, _z, _w] => {
            PAIR
        }
        _ => HIGH_CARD
    };
}

fn card_rank(card: char) -> Result<u8, InvalidCard> {
    if card >= '2' && card <= '9' {
        return Ok(card as u8 - '0' as u8)
    }
    match card {
        'T' => Ok(10),
        'J' => Ok(1),
        'Q' => Ok(12),
        'K' => Ok(13),
        'A' => Ok(14),
        _ => Err(InvalidCard),
    }
}

fn parse_cards(lex: &mut Lexer<Token>) -> Option<[u8; 5]> {
    let cards = lex.slice();
    let ranks: Vec<u8> = cards.chars().map(|f| {
        card_rank(f).expect("Invalid card")
    }).collect();
    if ranks.len() == 5 {
        Some([ranks[0], ranks[1], ranks[2], ranks[3], ranks[4]])
    }
    else {
        None
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[regex("[0-9]+", priority = 1, callback = |lex| lex.slice().parse().ok())]
    Bid(i32),
    #[regex("[AKQJT2-9]{5}", priority = 2, callback = parse_cards)]
    Card([u8; 5]),
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");

    let mut lex = Token::lexer(&input);

    let mut hands: Vec<Hand> = Vec::new();
    loop {
        let cards = lex.next();
        if cards == None {
            break;
        }
        let cards = cards.unwrap();
        let bid = lex.next().unwrap();
        // println!("Token {:?} - bid {:?}", cards, bid);
        let cards = match cards {
            Ok(Token::Card(c)) => c,
            _ => panic!("No card available {:?}", cards),
        };
        let bid = match bid {
            Ok(Token::Bid(b)) => b,
            _ => panic!("No bid available {:?}", cards),
        };
        hands.push(Hand {
            cards,
            bid,
        });
    }

    // ew...
    hands.sort_by(|a, b| {
        match hand_rank(a.cards).cmp(&hand_rank(b.cards)) {
            Ordering::Equal => match a.cards[0].cmp(&b.cards[0]) {
                Ordering::Equal => match a.cards[1].cmp(&b.cards[1]) {
                    Ordering::Equal => match a.cards[2].cmp(&b.cards[2]) {
                        Ordering::Equal => match a.cards[3].cmp(&b.cards[3]) {
                            Ordering::Equal => a.cards[4].cmp(&b.cards[4]),
                            otherwise => otherwise,
                        }
                        otherwise => otherwise,
                    }
                    otherwise => otherwise,
                },
                otherwise => otherwise,
            }
            otherwise => otherwise,
        }
    });

    // println!("{:?}", hands);
    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        total += hand.bid * (i as i32 + 1);
    }

    println!("Total score: {}", total);
    assert_eq!(245576185, total);
}
