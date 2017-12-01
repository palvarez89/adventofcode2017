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


    // Print text to the console
    println!("Hello World!");
    println!("In file {}", filename);

    let f = File::open(filename).expect("file not found");

    let f = BufReader::new(f);

    for line in f.lines() {
        calculate_captcha2(line.unwrap());
    }



}

fn calculate_captcha1(captcha: String) {
    let captcha_chars: Vec<_> = captcha.chars().collect();
    let mut total: u32 = 0;
    for x in 0..captcha.len() {
        let current = captcha_chars[x];
        let next = captcha_chars[(x+1) % captcha.len() ];
        if current == next {
            total += current.to_digit(10).unwrap();
        }
    }
    println!("{}", captcha);
    println!("{}", total);
}

fn calculate_captcha2(captcha: String) {
    let captcha_chars: Vec<_> = captcha.chars().collect();
    let half = captcha.len() / 2;

    let mut total: u32 = 0;
    for x in 0..captcha.len() {
        let current = captcha_chars[x];
        let next = captcha_chars[(x+half) % captcha.len() ];
        if current == next {
            total += current.to_digit(10).unwrap();
        }
    }
    println!("{}", captcha);
    println!("{}", total);
}

