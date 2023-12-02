#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io, io::Read};

static NUMBERS: [(&str, i32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_first(line: &str) -> i32 {
    for i in 0..line.len() {
        let sub = &line[i..line.len()];
        if sub.bytes().nth(0).unwrap() >= '1' as u8 && sub.bytes().nth(0).unwrap() <= '9' as u8 {
            return (sub.bytes().nth(0).unwrap() - '0' as u8).into();
        }
        for (text, value) in NUMBERS {
            if sub.starts_with(text) {
                return value;
            }
        }
    }

    panic!("Unable to parse start line {}", line);
}

fn find_last(line: &str) -> i32 {
    for i in (0..line.len()).rev() {
        let sub = &line[0..=i];
        if sub.bytes().nth(i).unwrap() >= '1' as u8 && sub.bytes().nth(i).unwrap() <= '9' as u8 {
            return (sub.bytes().nth(i).unwrap() - '0' as u8).into();
        }
        for (text, value) in NUMBERS {
            if sub.ends_with(text) {
                return value;
            }
        }
    }

    panic!("Unable to parse end line {}", line);
}

fn main() {
    let input = read_file("inputs/day1_1.txt").expect("Unable to read file");

    let mut total = 0;
    let mut index = 1;
    for line in input.lines() {
        let first = find_first(line);
        let last = find_last(line);
        let value = first * 10 + last;
        println!("{}: In {} found {},{}", index, line, first, last);
        total += value;
        index += 1;
    }
    println!("Total is {}", total);
}
