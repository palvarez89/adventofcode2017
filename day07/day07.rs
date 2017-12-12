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
    let root = get_root_i_tree(i_tree);

    let tree = construct_tree(filename);
    let weights = get_weights(filename);

    calculate_branch_weight(&root, &tree, &weights);
}


fn print_branch_weights(root: String, tree: HashMap<String, Vec<String>>,weights: HashMap<String, i32>){

    let main_branches_roots = tree.get(&root).unwrap();

    for i in main_branches_roots.iter() {
        println!("{}", i);
        let branch_weight = calculate_branch_weight(i, &tree, &weights);
        println!("{}", branch_weight);

    }
}

fn calculate_branch_weight(root: &String, tree: &HashMap<String, Vec<String>>,weights: &HashMap<String, i32>) -> i32 {

    let mut weight = weights.get(root).unwrap().clone();
    let childs_o = tree.get(root);

    let mut childs_weight = -1;
    let mut child_name = &String::new();
    if childs_o.is_some() {

        let childs = childs_o.unwrap();
        for ch in childs {
            
            let child_weight = calculate_branch_weight(ch, tree, weights);
            if childs_weight == -1 {
                childs_weight = child_weight;
                child_name = ch;
            }
            else {
                if childs_weight != child_weight{
                    println!("PROBLEM in {}", root);
                    println!("Weight recorded (child {}) {}, new ({}) {}", child_name, childs_weight, ch, child_weight);
                }
            }

            weight += child_weight;
        }
    }
    println!("{}", weight);
    (weight)
}

fn get_root_i_tree(i_tree: HashMap<String, String>) -> String {
    let mut value;
 
    value = i_tree.values().next().unwrap();
    for i in 0..i_tree.len() {
        let parent = match i_tree.get(value) {
            Some(s) => s,
            None => value,
        };
        if parent == value {
            println!("BASE PROGRAM FOUND: {}", parent);
            break;
        }
        println!("Node -{}- has parent -{}-", value, parent);
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
        let vec: Vec<&str> = content.split('>').collect();
        let parent_info: Vec<&str> = vec[0].split_whitespace().collect();

        // It's not a leaf
        if vec.len()>1 {
            let mut childs: Vec<&str> = vec[1].split(',').collect();
            for i in 0..childs.len() {
                let parent = parent_info[0].to_owned();
                tree.insert(childs[i].trim().to_owned(), parent);
                childs[i] = childs[i].trim();
            }
        }
    }
    (tree)
}


fn get_weights(filename: &String) -> HashMap<String, i32> {


    let mut weights: HashMap<String, i32> = HashMap::new();

    let f = File::open(filename).expect("file not found");
    let f = BufReader::new(f);

    for line in f.lines() {
        let content = line.unwrap();
        let vec: Vec<&str> = content.split('>').collect();
        let parent_info: Vec<&str> = vec[0].split_whitespace().collect();
        let parent = parent_info[0].to_owned();

        let weight_str: &str = parent_info[1].split('(').nth(1).unwrap().split(')').nth(0).unwrap();

        let weight: i32 = weight_str.parse().unwrap();
        println!("weight of {} is {}", &parent, weight);
        weights.insert(parent, weight);

        let mut clean_childs: Vec<String> = Vec::new();

    }

    (weights)

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
