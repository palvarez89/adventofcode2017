use std::env;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
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

    let i_tree = construct_inverse_tree(filename);
    get_root_i_tree(i_tree);
    //let tree = construct_tree(filename);

}


fn get_root_i_tree(i_tree: HashMap<String, String>) -> String {
    let mut value;
 
    value = i_tree.values().next().unwrap();
    for i in 0..i_tree.len() {
        let parent = match i_tree.get(value) {
            Some(s) => s,
            None => value,
        };
        println!("It was -{}- moving to -{}-", value, parent);
        if parent == value {
            println!("BASE PROGRAM FOUND: {}", parent);
            break;
        }
        value = parent;

    }
  
    (value.clone())
}

fn construct_inverse_tree(filename: &String) -> HashMap<String, String> {

    let mut tree: HashMap<String, String> = HashMap::new();

    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    for line in f.lines() {
        let content = line.unwrap();
        println!("{}", content);
        let vec: Vec<&str> = content.split('>').collect();
        let parent_info: Vec<&str> = vec[0].split_whitespace().collect();
        println!("1 +++{}+++", parent_info[0]);

        // It's not a leaf
        if vec.len()>1 {
            println!("2 {}", vec[1]);
            let mut childs: Vec<&str> = vec[1].split(',').collect();
            for i in 0..childs.len() {
                let parent = parent_info[0].to_owned();
                println!("{} -> {}", childs[i], &parent);
                tree.insert(childs[i].trim().to_owned(), parent);
                childs[i] = childs[i].trim();
                println!("---{}----",childs[i]);
            }
        }
    }
    (tree)
}

fn construct_tree(filename: &String) -> HashMap<String, Vec<String>> {

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();

    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    for line in f.lines() {
        let content = line.unwrap();
        println!("{}", content);
        let vec: Vec<&str> = content.split('>').collect();
        let parent_info: Vec<&str> = vec[0].split_whitespace().collect();
        let parent = parent_info[0].to_owned();
        println!("1 +++{}+++", parent);
        let mut clean_childs: Vec<String> = Vec::new();

        // It's not a leaf
        if vec.len()>1 {
            println!("2 {}", vec[1]);
            let mut childs: Vec<&str> = vec[1].split(',').collect();
            for i in 0..childs.len() {
                clean_childs.push(childs[i].trim().to_owned());
                childs[i] = childs[i].trim();
                println!("---{}----",childs[i]);
            }
        }
        tree.insert(parent, clean_childs);  
    }

    (tree)
}
