use std::{io::{self, Read}, fs::File};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn next_in_seq(main_seq: &Vec<i32>) -> i32 {
    let mut next_value = 0;
    let mut sequences: Vec<Vec<i32>> = Vec::new();
    sequences.push(main_seq.clone());
    let mut current_sequence: Vec<i32> = main_seq.clone();

    loop {
        let mut previous_num = current_sequence[0];
        let mut next_seq: Vec<i32> = Vec::new();
        for i in current_sequence.iter().skip(1) {
            next_seq.push(i - previous_num);
            previous_num = *i;
        }

        let all_zero = next_seq.iter().find(|n| **n != 0) == None;
        if all_zero {
            break;
        }

        current_sequence = next_seq.clone();
        sequences.push(next_seq);
    }

    for sequence in sequences.iter_mut().rev() {
        let last_value = sequence.last().unwrap();
        next_value = last_value + next_value;
        sequence.push(next_value);
    }

    next_value
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");

    let mut final_result_next = 0;
    let mut final_result_prev = 0;
    for line in input.lines() {
        let seq: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let next = next_in_seq(&seq);
        let mut seq_reverse: Vec<i32> = seq.clone();
        seq_reverse.reverse();
        let prev = next_in_seq(&seq_reverse);

        println!("Next value is {}, prev value is {}", next, prev);
        final_result_next += next;
        final_result_prev += prev;
    }

    println!("Final result is {}", final_result_next);
    println!("Final result is {}", final_result_prev);
    assert_eq!(1479011877, final_result_next);
    assert_eq!(973, final_result_prev);
}
