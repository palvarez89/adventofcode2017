use std::env;
use std::fs::File;

extern crate regex;
use regex::Regex;

use std::io::prelude::*;
use std::io::{BufReader};

fn main() {
    // The statements here will be executed when the compiled binary is called
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("i need a filename as argument!!");
        std::process::exit(1) 
    }

    let filename = &args[1];

    calculate_checksum2(filename);

}


fn calculate_checksum1(filename: &String) {
    println!("Reading from file {}", filename);

    let f = File::open(filename).expect("file not found");

    let f = BufReader::new(f);

    let mut checksum = 0;

    for line in f.lines() {
        let content = line.unwrap();
        println!("{}", content);
        checksum += get_diff_min_max(content);
    }

    println!("CHECKSUM {}", checksum);
}

fn calculate_checksum2(filename: &String) {
    println!("Reading from file {}", filename);

    let f = File::open(filename).expect("file not found");

    let f = BufReader::new(f);

    let mut checksum = 0;

    for line in f.lines() {
        let content = line.unwrap();
        println!("{}", content);
        checksum += get_divisable_result(content);
    }

    println!("CHECKSUM {}", checksum);
}

fn get_divisable_result(line: String) -> i32 {

    let vec: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let vec_size = vec.len();
    let mut result = 0;

    for x in 0..vec_size {
        for y in 0..vec_size {
            if x == y {
                continue;
            }
            if vec[x] % vec[y] == 0 {
                println!("Found: {}/{}", vec[x], vec[y]); // x: i32
                result = vec[x] / vec[y]
            }
        }
    }
    if result == 0 {
        println!("WARNING, it's 0????");
    }

    (result)

}

fn get_diff_min_max(line: String) -> i32 {

    let re = Regex::new(r"[ \t]+").unwrap();
    let fields = re.split(&line);
    let mut min: Option<i32> = None;
    let mut max: Option<i32> = None;
    
    for f in fields {
        let current = f.parse::<i32>().unwrap();
        if let Some(value) = min {
            if current < value {
                min = Some(current);
            }
        }
        else {
            min = Some(current);
        }
        if let Some(value) = max {
            if current > value {
                max = Some(current);
            }
        }
        else {
            max = Some(current);
        }
    }
    
    let realmin = min.unwrap_or(0);
    let realmax = max.unwrap_or(0);
    let diff = realmax - realmin;

    println!("min {} max {}, diff {}", realmin, realmax, diff);
    (diff)
}
