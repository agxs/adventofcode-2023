use std::{io::{self, Read}, fs::File};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

struct Route {
    current: Coord,
    next: Coord,
}

fn find_start(map: &Vec<Vec<char>>) -> Coord {
    for (y, line) in map.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                return Coord { x, y };
            }
        }
    }

    panic!("No start found");
}

fn first_part(start: &Coord, map: &Vec<Vec<char>>) -> Coord {
    if start.x > 0 {
        let left = map[start.y][start.x - 1];
        if left == '-' || left == 'L' || left == 'F' {
            return Coord { x: start.x - 1, y: start.y };
        }
    }
    // Yeah, yeah, none of my input data has S at the top, right or botton of the map...
    let right = map[start.y][start.x + 1];
    if right == '-' || right == 'J' || right == '7' {
        return Coord { x: start.x + 1, y: start.y };
    }
    let top = map[start.y-1][start.x];
    if top == '|' || top == 'F' || top == '7' {
        return Coord { x: start.x , y: start.y - 1};
    }
    let bottom = map[start.y+1][start.x];
    if bottom == '|' || bottom == 'L' || bottom == 'J' {
        return Coord { x: start.x , y: start.y + 1};
    }

    panic!("No connections to start found");
}

fn next_part(previous: &Coord, current: &Coord, map: &Vec<Vec<char>>) -> Coord {
    let current_char = map[current.y][current.x];
    if previous.x == current.x && previous.y < current.y {
        // moving down
        let next = match current_char {
            '|' => Coord { x: current.x, y: current.y + 1},
            'J' => Coord { x: current.x - 1, y: current.y },
            'L' => Coord { x: current.x + 1, y: current.y },
            _ => panic!("moving down error"),
        };
        // return Route { current: *current, next };
        return next;
    }
    else if previous.x == current.x && previous.y > current.y {
        // moving up
        let next = match current_char {
            '|' => Coord { x: current.x, y: current.y - 1},
            '7' => Coord { x: current.x - 1, y: current.y },
            'F' => Coord { x: current.x + 1, y: current.y },
            _ => panic!("moving up error"),
        };
        // return Route { current: *current, next };
        return next;
    }
    else if previous.x < current.x && previous.y == current.y {
        // moving right
        let next = match current_char {
            '-' => Coord { x: current.x + 1, y: current.y },
            'J' => Coord { x: current.x, y: current.y - 1 },
            '7' => Coord { x: current.x, y: current.y + 1},
            _ => panic!("moving right error"),
        };
        // return Route { current: *current, next };
        return next;
    }
    else if previous.x > current.x && previous.y == current.y {
        // moving left
        let next = match current_char {
            '-' => Coord { x: current.x - 1, y: current.y },
            'L' => Coord { x: current.x, y: current.y - 1},
            'F' => Coord { x: current.x, y: current.y + 1 },
            _ => panic!("moving left error"),
        };
        // return Route { current: *current, next };
        return next;
    }

    panic!("wat");
}

/**
 * Casts a ray from the left and counts the intersections with the route. Odd numbers
 * mean that we're inside the route, even numbers mean we're outside the route.
 */
fn point_in_loop(point: &Coord, route: &Vec<Coord>, map: &Vec<Vec<char>>) -> bool {
    let mut crossing_count = 0;
    // these two booleans figure out if we're crossing a horizontal part of the route
    // that intersects with our 'ray'
    let mut has_up = false;
    let mut has_down = false;
    for x in 0..=point.x {
        let c = map[point.y][x];
        match c {
            '.' => continue,
            'S' => continue,
            '|' => if route.contains(&Coord { x, y: point.y }) {
                crossing_count += 1;
            },
            '-' => continue,
            'F' => {
                if !route.contains(&Coord { x, y: point.y }) {
                    continue;
                }
                has_down = true;
            },
            'L' => {
                if !route.contains(&Coord { x, y: point.y }) {
                    continue;
                }
                has_up = true;
            },
            'J' => {
                if !route.contains(&Coord { x, y: point.y }) {
                    continue;
                }
                if has_down {
                    crossing_count += 1;
                }
                has_up = false;
                has_down = false;
            }
            '7' => {
                if !route.contains(&Coord { x, y: point.y }) {
                    continue;
                }
                if has_up {
                    crossing_count += 1;
                }
                has_up = false;
                has_down = false;
            }
            _ => panic!("wat"),
        }
    }

    // odd numbers of path crossings mean the point is inside
    crossing_count % 2 == 1
}

fn get_start_symbol(start: &Coord, end: &Coord) -> char {
    if start.x < end.x {
        if start.y < end.y {
            return '7';
        }
        else if (start.y == end.y) {
            return '-';
        }
        else {
            return 'J';
        }
    }
    else if start.x > end.x {
        if start.y < end.y {
            return 'F';
        }
        else {
            return 'L';
        }
    }

    '|'
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        map.push(line_vec);
    }


    let start = find_start(&map);
    let first = first_part(&start, &map);

    let mut full_loop: Vec<Coord> = vec![start, first];

    let mut previous = start;
    let mut current = first;

    let mut steps = 1;
    loop {
        if current == start {
            break;
        }

        let next = next_part(&previous, &current, &map);
        previous = current;
        current = next;
        steps += 1;
        full_loop.push(current);
    }

    println!("Start: {:?}, first: {:?}", start, first);
    let half_way = (steps as f32 / 2_f32).ceil() as i32;
    println!("Steps: {}", half_way);
    // println!("Full route: {:?}", full_loop);
    assert_eq!(6806, half_way);

    // Replace the start symbol with whats its actual symbol is
    // This lets us check the horizontal intersections when we're counting if a point is
    // inside the loop or not
    let start_symbol = get_start_symbol(&full_loop[1], &full_loop[full_loop.len() - 2]);
    println!("Start symbol is {}", start_symbol);
    map[start.y][start.x] = start_symbol;

    let mut inner_count = 0;
    for (y, v) in map.iter().enumerate() {
        for (x, c) in v.iter().enumerate() {
            if full_loop.contains(&Coord { x, y }) {
                continue;
            }
            if point_in_loop(&Coord { x, y }, &full_loop, &map) {
                inner_count += 1;
            }
        }
    }

    println!("Inner loop count: {}", inner_count);
    assert_eq!(449, inner_count);
}
