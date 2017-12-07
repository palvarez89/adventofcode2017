use std::env;
use std::fs::File;
use std::collections::HashMap;

use std::io::prelude::*;
use std::io::{BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("i need a filename as argument!!");
        std::process::exit(1) 
    }

    let filename = &args[1];

    run_program(filename);

}


fn run_program(filename: &String) {
    println!("Reading from file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Unable to read the file");

    let mut memory: Vec<i32>= contents.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut visited: HashMap<Vec<i32>, i32> = HashMap::new();
    let memory_len = memory.len();

    let mut steps = 0;
    for i in 0..memory_len {
        print!("{}",memory[i]);
    }
    println!();

    while !visited.contains_key(&memory) {
        visited.insert(memory.clone(), steps);
        let to_move = find_bank_to_move(&memory);
        redistribute_bank(&mut memory, to_move);
        for i in 0..memory_len {
            print!("{}",memory[i]);
        }
        println!();
        steps += 1;
    }
    
    let step_when_introduced = visited.get(&memory);
    println!("Introduced in {}", step_when_introduced.unwrap());

    println!("STEPS NEEDED: {}", steps);
    println!("Cycles in loop: {}", steps - step_when_introduced.unwrap());

}

fn redistribute_bank(memory: &mut Vec<i32>, pos: usize) {
    let mem_len = memory.len();
    let mut bank = memory[pos];
    memory[pos] = 0;
    
    let mut i = pos;
    while bank > 0 {
        i = (i+1) % mem_len;
        memory[i] += 1;
        bank -= 1;
    }

}

fn find_bank_to_move(memory: &Vec<i32>) -> usize {
    let mut pos_to_move: usize = 0;
    let mut max: i32 = 0;

    for i in 0..memory.len() {
        if memory[i] > max {
            max = memory[i];
            pos_to_move = i;
        }
    }
    (pos_to_move)
}
