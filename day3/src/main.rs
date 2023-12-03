#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashMap;
use std::{fs::File, io::{self, Read}};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn is_symbol(s: char) -> bool {
    s != '.' && !s.is_digit(10)
}
fn is_gear(s: char) -> bool {
    s == '*'
}

struct SchematicToken {
    id: i32,
    start: usize,
    end: usize,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct SymbolLocation {
    x: usize,
    y: usize,
}

fn walk_around_number(token: &SchematicToken, y: usize, schematics: &Vec<Vec<char>>, symbol_check: fn(char) -> bool) -> Vec<SymbolLocation> {
    let start = if token.start == 0 { 0 } else { token.start - 1 };
    let end = (schematics[y].len() - 1).min(token.end);
    let mut symbol_locations: Vec<SymbolLocation> = Vec::new();
    if symbol_check(schematics[y][start]) {
        symbol_locations.push(SymbolLocation { x: start, y });
    }
    if symbol_check(schematics[y][end]) {
        symbol_locations.push(SymbolLocation { x: end, y });
    }

    if y != 0 {
        for i in start..=end {
            let c = schematics[y-1][i];
            if symbol_check(c) {
                symbol_locations.push(SymbolLocation { x: i, y: y - 1 });
            }
        }
    }
    if y + 1 < schematics.len() {
        for i in start..=end {
            let c = schematics[y+1][i];
            if symbol_check(c) {
                symbol_locations.push(SymbolLocation { x: i, y: y + 1 });
            }
        }
    }

    symbol_locations
}

fn parse_next_number(line: &Vec<char>, skip: usize) -> Option<SchematicToken> {
    let mut part_id = String::new();
    let mut found_digit = false;
    let mut start = 0;
    for (index, c) in line.iter().enumerate().skip(skip) {
        if !c.is_digit(10) {
            if found_digit {
                return Some(SchematicToken {
                    id: part_id.parse().expect(&format!("Can't parse number {}", part_id)),
                    start,
                    end: index,
                });
            }

            continue;
        }

        if !found_digit {
            start = index;
        }
        found_digit = true;
        part_id.push(*c);
    }

    if found_digit {
        return Some(SchematicToken {
            id: part_id.parse().expect(&format!("Can't parse number {}", part_id)),
            start,
            end: line.len()
        });
    }
    None
}

fn main() {
    let mut schematics: Vec<Vec<char>> = Vec::new();
    let mut gear_locations: HashMap<SymbolLocation, Vec<i32>> = HashMap::new();
    let input = read_file("input.txt").expect("Unable to read file");

    let mut schematic_total = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        schematics.push(chars);
    }

    for (y, line) in schematics.iter().enumerate() {
        let schematic_id = String::new();
        let mut index = 0;
        while index < line.len() {
            let token = parse_next_number(line, index);
            match token {
                Some(t) => {
                    // println!("Token {} found", t.id);
                    index = t.end;
                    if walk_around_number(&t, y, &schematics, is_symbol).len() > 0 {
                        // println!("  {} is part number", t.id);
                        schematic_total += t.id;
                    }
                    let gears = walk_around_number(&t, y, &schematics, is_gear);
                    for gear in gears.iter() {
                        gear_locations.entry(*gear).or_insert(Vec::new()).push(t.id);
                    }
                },
                None => break,
            }
        }
    }

    println!("Schematic total: {}", schematic_total);
    assert_eq!(553079, schematic_total, "Incorrect part numbers");
    let gear_ratios: i32 = gear_locations.iter()
        .filter_map(|(&gear, ids)| {
            if ids.len() == 2 {
                Some(ids[0] * ids[1])
            } else {
                None
            }
        })
        .sum();
    println!("Sum of gear ratios: {}", gear_ratios);
    assert_eq!(84363105, gear_ratios, "Incorret gear ratios");
}
