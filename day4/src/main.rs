#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io::{self, Read}, fs::File, collections::{HashSet, HashMap}};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn card_points(input: &str) {

    let mut total_points = 0;
    for line in input.lines() {
        println!("Line {}", line);
        let mut colon = line.split(":");
        let card = colon.next().expect("Blank line?");
        let numbers = colon.next().unwrap();

        let mut pipe = numbers.split("|");
        let winners = pipe.next().unwrap();
        let cards = pipe.next().unwrap();

        let mut number_matches: Vec<&str> = Vec::new();
        let winning_numbers: HashSet<&str> = winners.split_whitespace().collect();
        for number in cards.split_whitespace() {
            if winning_numbers.contains(number) {
                number_matches.push(number);
            }
        }

        println!("  Matches {:?}", number_matches);

        let points = if number_matches.len() > 0 {
            2_i32.pow(number_matches.len() as u32 - 1)
        }
        else {
            0
        };

        println!("  Points {}", points);

        total_points += points;
    }

    println!("Scratch card winnings: {}", total_points);
    assert_eq!(26426, total_points);
}

fn card_count(input: &str) {
    let mut card_count: HashMap<usize, i32> = input.lines().into_iter().enumerate().map(|(i, x)| (i + 1, 1) ).collect();

    for line in input.lines() {
        let mut colon = line.split(":");
        let card = colon.next().expect("Blank line?");
        let numbers = colon.next().unwrap();

        let mut card_text = card.split_whitespace();
        card_text.next();
        let card_id: usize = card_text.next().unwrap().parse().unwrap();

        let mut pipe = numbers.split("|");
        let winners = pipe.next().unwrap();
        let cards = pipe.next().unwrap();

        let mut number_matches: Vec<&str> = Vec::new();
        let winning_numbers: HashSet<&str> = winners.split_whitespace().collect();
        for number in cards.split_whitespace() {
            if winning_numbers.contains(number) {
                number_matches.push(number);
            }
        }

        let end_index = card_count.len().min(card_id as usize + number_matches.len());
        if card_id + 1 > card_count.len() {
            break;
        }
        let count_at_current: i32 = *card_count.get(&card_id).expect(&format!("Current Value {} is missing", card_id));
        for i in card_id+1..=end_index {
            let current_count = card_count.get(&i).expect(&format!("Value {} is missing", i));
            card_count.insert(i, current_count + count_at_current);
        }
    }

    let total_cards: i32 = card_count.iter().map(|(k, v)| v).sum();
    println!("Total card: {}", total_cards);
    assert_eq!(6227972, total_cards);
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");

    card_points(&input);
    card_count(&input);
}
