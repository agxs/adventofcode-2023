use std::{io::{self, Read}, fs::File, collections::HashMap, process::exit};

use logos::Logos;
use num_integer::Integer;


fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f,\(\)=]+")]
enum Token {
    #[regex("[0-9A-Z]+")]
    Node,
}

#[derive(Clone, Copy)]
struct Node<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}

fn main() {
    let input = read_file("input.txt").expect("Unable to read file");

    let mut lex = Token::lexer(&input);

    let mut nodes: HashMap<&str, Node> = HashMap::new();

    lex.next();
    let path = lex.slice();

    println!("Path is {}", path);
    loop {
        let result = lex.next();
        let parent = match result {
            Some(_x) => lex.slice(),
            None => break,
        };

        lex.next();
        let left = lex.slice();

        lex.next();
        let right = lex.slice();

        nodes.insert(parent, Node { id: parent, left, right });

        println!("Node: {} -> {}, {}", parent, left, right);
    }

    // The below brute forces the answer. But it's a lot of steps...

    // let starting_nodes: Vec<&str> = nodes.keys().filter(|k| k.ends_with("A")).cloned().collect();
    // let mut current_nodes: Vec<Node> = starting_nodes.iter().map(|n| nodes.get(n).expect(&format!("Missing node {}", n))).cloned().collect();
    // let mut step_count = 0_u64;

    // loop {
    //     for c in path.chars().into_iter() {
    //         step_count += 1;

    //         let old_nodes = current_nodes.clone();
    //         for (i, mut current_node) in old_nodes.iter().enumerate() {
    //             current_node = match c {
    //                 'L' => nodes.get(current_node.left).expect(&format!("Unknown node {}", c)),
    //                 'R' => nodes.get(current_node.right).expect(&format!("Unknown node {}", c)),
    //                 _ => panic!("Unknown character in path {}", c),
    //             };
    //             current_nodes[i] = current_node.clone();
    //         }
    //         let at_end = current_nodes.iter().filter(|n| n.id.ends_with("Z")).count() == starting_nodes.len();
    //         if step_count % 10000000 == 0 {
    //             println!("Searching... step {}", step_count);
    //         }
    //         if at_end {
    //             println!("Step count: {}", step_count);
    //             // 13771
    //             exit(0);
    //         }
    //     }
    // }

    // Really this should read the source data to get the starting nodes but whatever
    let length1 = find_path_length(path, nodes.get("GSA").unwrap(), &nodes);
    let length2 = find_path_length(path, nodes.get("DLA").unwrap(), &nodes);
    let length3 = find_path_length(path, nodes.get("MLA").unwrap(), &nodes);
    let length4 = find_path_length(path, nodes.get("MQA").unwrap(), &nodes);
    let length5 = find_path_length(path, nodes.get("AAA").unwrap(), &nodes);
    let length6 = find_path_length(path, nodes.get("JGA").unwrap(), &nodes);
    println!("Path1 is {}", length1);
    println!("Path2 is {}", length2);
    println!("Path3 is {}", length3);
    println!("Path4 is {}", length4);
    println!("Path5 is {}", length5);
    println!("Path6 is {}", length6);
    let l1 = length1.lcm(&length2);
    let l2 = l1.lcm(&length3);
    let l3 = l2.lcm(&length4);
    let l4 = l3.lcm(&length5);
    let final_lcm = l4.lcm(&length6);
    println!("Final path {}", final_lcm);
    assert_eq!(13129439557681, final_lcm);
}

fn find_path_length(path: &str, starting_node: &Node, nodes: &HashMap<&str, Node>) -> u64 {
    let mut step_count = 0;
    let mut current_node = starting_node;
    loop {
        for c in path.chars().into_iter() {
            step_count += 1;

            current_node = match c {
                'L' => nodes.get(current_node.left).expect(&format!("Unknown node {}", c)),
                'R' => nodes.get(current_node.right).expect(&format!("Unknown node {}", c)),
                _ => panic!("Unknown character in path {}", c),
            };

            if current_node.id.ends_with('Z') {
                return step_count;
            }
        }
    }
}
