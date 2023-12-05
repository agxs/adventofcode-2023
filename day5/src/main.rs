#![allow(dead_code)]
#![allow(unused_variables)]

use std::{io::{self, Read}, fs::File, collections::{HashSet, HashMap}};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
struct Map {
    source: i64,
    destination: i64,
    range: i64,
}

impl Map {
    fn map(&self, input: i64) -> i64 {
        (input - self.source) + self.destination
    }

    fn in_range(&self, input: i64) -> bool {
        self.source <= input && input < self.source + self.range
    }
}

fn read_mapping<'a, I: Iterator<Item = &'a str>>(iter: &mut I) -> Vec<Map> {
    let mut mapping: Vec<Map> = Vec::new();
    iter.next(); //title

    for line in iter {
        if line == "" {
            break;
        }

        let mapping_vec: Vec<&str> = line.split_whitespace().collect();
        if mapping_vec.len() != 3 {
            panic!("Mapping doesn't have 3 numbers: {:?}", mapping_vec);
        }
        let destination: i64 = mapping_vec[0].parse().unwrap();
        let source: i64 = mapping_vec[1].parse().unwrap();
        let range: i64 = mapping_vec[2].parse().unwrap();

        mapping.push(Map {
            source,
            destination,
            range,
        });
    }

    mapping
}

fn do_mapping(value: i64, mappings: &Vec<Map>, name: &str) -> i64 {
    for mapping in mappings {
        if mapping.in_range(value) {
            return mapping.map(value);
        }
    }

    value
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");
    let mut lowest_location = std::i64::MAX;

    let mut input_iter = input.lines();
    let seed_str = input_iter.next().unwrap();
    input_iter.next(); // blank space

    let seed_to_soil: Vec<Map> = read_mapping(&mut input_iter);
    let soil_to_fert: Vec<Map> = read_mapping(&mut input_iter);
    let fert_to_water: Vec<Map> = read_mapping(&mut input_iter);
    let water_to_light: Vec<Map> = read_mapping(&mut input_iter);
    let light_to_temp: Vec<Map> = read_mapping(&mut input_iter);
    let temp_to_humidity: Vec<Map> = read_mapping(&mut input_iter);
    let humidity_to_location: Vec<Map> = read_mapping(&mut input_iter);

    println!("S to S {:?}", seed_to_soil);
    println!("S to F {:?}", soil_to_fert);
    println!("F to W {:?}", fert_to_water);
    println!("W to L {:?}", water_to_light);
    println!("L to T {:?}", light_to_temp);
    println!("T to H {:?}", temp_to_humidity);
    println!("H to L {:?}", humidity_to_location);

    let seed_split: Vec<&str> = seed_str.split(":").collect();
    let seeds: Vec<i64> = seed_split[1].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut seed_pairs: Vec<(i64, i64)> = Vec::new();
    let mut seed_pair_iter = seeds.iter();
    loop {
        let start = match seed_pair_iter.next() {
            Some(t) => t,
            None => break,
        };
        let end = seed_pair_iter.next().unwrap() + start;
        seed_pairs.push((*start, end));
    }
    for (start, end) in seed_pairs.iter() {
        for s in *start..*end {
            let value = do_mapping(s, &seed_to_soil, "Seed To Soil");
            let value = do_mapping(value, &soil_to_fert, "Soil To Fert");
            let value = do_mapping(value, &fert_to_water, "Fert To Water");
            let value = do_mapping(value, &water_to_light, "Water To Light");
            let value = do_mapping(value, &light_to_temp, "Light To Temp");
            let value = do_mapping(value, &temp_to_humidity, "Temp To Humidity");
            let value = do_mapping(value, &humidity_to_location, "Humidity To Location");

            lowest_location = lowest_location.min(value);
        }
    }

    println!("Lowest id is {}", lowest_location);
    assert_eq!(79874951, lowest_location);
}
