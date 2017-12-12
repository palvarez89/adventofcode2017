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

    let mut deep = 0;
    let mut in_trash = false;
    let mut ignore_next = false;
    let mut score = 0;
    let mut garbage = 0;

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Unable to read the file");
    for ch in contents.chars() {
        if ignore_next {
            ignore_next = false;
            println!("-i- {}", ch);
            continue;
        }
        if ch == '!' {
            ignore_next = true;
            println!("{}", ch);
            continue;
        }
        if in_trash {
            match ch {
                '>' => { in_trash = false; println!("trashmode off")},
                _ => {println!("-t- {}", ch); garbage += 1},
            }
        }
        else {
            println!("{}", ch);
            match ch {
                '{' => { deep += 1; println!("opened")},
                '}' => { score += deep; deep -= 1; println!("closed")},
                '<' => { in_trash = true; println!("trashmode")},
                _ => println!("{}", ch),
            }
        }
        if deep < 0 { println!("BOOM");}

    }
    println!("SCORE is {}", score);
    println!("GARBAGE is {}", garbage);

}
