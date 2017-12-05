use std::env;
use std::fs::File;
use std::collections::HashSet;
use std::iter::Iterator;
use std::iter::FromIterator;

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

    get_number_secure_passwords(filename);

}

fn get_number_secure_passwords(filename: &String) {
    println!("Reading from file {}", filename);

    let f = File::open(filename).expect("file not found");

    let f = BufReader::new(f);

    let mut secure_passwords  = 0;

    for line in f.lines() {
        let content = line.unwrap();
        println!("{}", content);
        if is_secure_password2(content) {
            secure_passwords += 1;
        }
    }

    println!("Num secure passwords {}", secure_passwords);
}

fn is_secure_password(pass: String) -> bool {

    let vec: Vec<&str> = pass.split_whitespace().collect();

    let vec_size = vec.len();
    let mut result = true;
    let mut words = HashSet::new();

    for x in 0..vec_size {
        if words.contains(vec[x]){
            println!("wasn't secure");
            result = false;
            break;
        }
        else {
            words.insert(vec[x]);
        }
    }
    (result)
}

fn is_secure_password2(pass: String) -> bool {

    let vec: Vec<&str> = pass.split_whitespace().collect();

    let vec_size = vec.len();
    let mut result = true;
    let mut words = HashSet::new();

    for x in 0..vec_size {
        let mut chars: Vec<char> = vec[x].chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        let s = String::from_iter(chars);

        if words.contains(&s){
            println!("wasn't secure");
            result = false;
            break;
        }
        else {
            words.insert(s);
        }
    }
    (result)
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
