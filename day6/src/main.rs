use std::{fs::File, io::{self, Read}};
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
enum Token {
    #[token("Time:")]
    Time,
    #[token("Distance:")]
    Distance,
    #[regex("[0-9]+", |lex| lex.slice().parse().ok())]
    Number(u64),
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");

    let lex = Token::lexer(&input);
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    // read all numbers into the `times` vec until the Distance: token is found
    // then read all the numbers into the `distances` vec
    let mut reading_times = true;
    for token in lex {
        match token {
            Ok(Token::Number(v)) => {
                if reading_times {
                    times.push(v);
                }
                else {
                    distances.push(v);
                }
            },
            Ok(Token::Distance) => {
                reading_times = false;
            },
            Ok(_t) => {},
            _ => break,
        }
    }

    println!("Times: {:?}", times);
    println!("Distances: {:?}", distances);
    assert_eq!(times.len(), distances.len());

    let mut  winning_total = 1_u64;

    for (i, total_time) in times.iter().enumerate() {
        let mut winning_count = 0;
        let total_distance = distances[i] as f64;
        for speed in 1..*total_time {
            let charge_time = speed as f64;
            let remaining_time = *total_time as f64 - charge_time;
            let distance = remaining_time * charge_time;
            if distance > total_distance {
                // println!("{} is a winner", speed);
                winning_count += 1;
            }
        }

        winning_total *= winning_count;
    }

    println!("Total winning posibilities: {}", winning_total);
    // pt1 1660968
    // pt2 26499773
}
