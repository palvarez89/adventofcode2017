use std::env;
use std::fs::File;

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

    run_program(filename);

}


fn run_program(filename: &String) {
    println!("Reading from file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Unable to read the file");
    println!("going to read");
    let mut all: Vec<i32>= contents.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let program_size = all.len();
    let mut address = 0;
    let mut steps = 0;

    println!("{}", contents);
    println!("Size = {}", program_size);
    while (address < program_size) & (address >= 0) {
        //println!("Current address {}", address);
        let jump = all[address];
        let next_address = all[address] + address as i32;

        //println!("next address {}", next_address);
        if jump >= 3 {
            all[address] -= 1;
        }
        else {
            all[address] += 1;
        }
        address = next_address as usize;
        steps += 1;
    }
    println!("Steps {}", steps);


    //for line in f.lines() {
     //   let content = line.unwrap();
     //   println!("{}", content);
     //   checksum += 1;
    //}


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
