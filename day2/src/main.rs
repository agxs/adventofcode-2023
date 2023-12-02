#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs::File, io, io::Read};
use std::collections::HashSet;

static TOTAL_RED: i32 = 12;
static TOTAL_GREEN: i32 = 13;
static TOTAL_BLUE: i32 = 14;

static COLOURS: [(&str, i32); 3] = [
    ("red", TOTAL_RED),
    ("green", TOTAL_GREEN),
    ("blue", TOTAL_BLUE),
];

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_valid_games(input: &str) {
    let mut valid_ids: HashSet<i32> = HashSet::new();
    let mut colour_powers: Vec<i32> = Vec::new();

    for line in input.lines() {
        // Each line looks like
        // Game 1: 5 red, 1 green; 6 red, 3 blue; 9 red; 1 blue, 1 green, 4 red; 1 green, 2 blue; 2 blue, 1 red

        let colon_split: Vec<&str> = line.split(":").collect();
        assert_eq!(2, colon_split.len(), "Line not formatted correctly for : split - {}", line);

        // game is `Game 1`
        // and
        // `5 red, 1 green; 6 red, 3 blue; 9 red; 1 blue, 1 green, 4 red; 1 green, 2 blue; 2 blue, 1 red``
        let game: Vec<&str> = colon_split[0].split_whitespace().collect();
        let game_id: i32 = game[1].parse().expect("Game id is not an integer");

        let sets = colon_split[1].split(";");
        let mut game_is_invalid = false;

        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        // Each set is `5 red, 1 green`
        for set in sets {
            let cubes = set.split(",");
            // Each cube_str is `5 red`
            for cube_str in cubes {
                // cube is `5` and `red`
                let cube: Vec<&str> = cube_str.split_whitespace().collect();
                assert_eq!(2, cube.len(), "Cube wrong size {}", cube_str);
                let count: i32 = cube[0].parse().expect("Invalid cube count");
                let colour = cube[1];

                match colour {
                    "red" => min_red = min_red.max(count),
                    "green" => min_green = min_green.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    _ => panic!("Unknown colour {} in game {}", colour, game_id)
                }

                for (colour_check, total) in COLOURS {
                    if colour_check == colour && count > total {
                        game_is_invalid = true;
                    }
                }
            }
        }

        colour_powers.push(min_red * min_green * min_blue);

        if !game_is_invalid {
            valid_ids.insert(game_id);
        }
    }

    let total: i32 = valid_ids.iter().sum();
    println!("Total of Valid Ids: {}", total);

    let colour_powers_total: i32 = colour_powers.iter().sum();
    println!("Total Colour Powers: {}", colour_powers_total);
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");
    find_valid_games(&input);
}