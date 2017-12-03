use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;

fn main() {
    // The statements here will be executed when the compiled binary is called
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("i need a number as argument!!");
        std::process::exit(1)
    }


    let number = &args[1].parse::<i32>().unwrap();

    println!("Number is {}", number);
    calculate_manhattan_distance(*number);
    calculate_neighbours_sum(*number);

}

fn calculate_neighbours_sum(target: i32) {
    let mut squares: HashMap<(i32,i32), i32> = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut incs = 1;
    let mut up_or_right = 1;
    let mut found = false;
    squares.insert((0,0),1);
    let mut result = 0;

    while !found {
        for j in 0..incs {
            x += up_or_right;
            let sum = sum_for_neighbours((x,y), &squares);
            squares.insert((x,y), sum);
            if sum > target {
                found = true;
                result = sum;
                break;
            }

        }
        if found {break;}

        for j in 0..incs {
            y += up_or_right;
            let sum = sum_for_neighbours((x,y), &squares);
            squares.insert((x,y), sum);
            if sum > target {
                found = true;
                result = sum;
                break;
            }
        }

        up_or_right = up_or_right * -1;
        incs += 1;



    }
    println!("The result is {}", result);

}

fn sum_for_neighbours(coords: (i32,i32), squares: &HashMap<(i32, i32), i32>) -> i32 {
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2{
            if (i == 0) & (j == 0) {
                continue;
            }
            let current_coord = (coords.0 + i, coords.1 +j);
            if squares.contains_key(&current_coord) {
                sum += squares.get(&current_coord).unwrap();
            }
        }

    }


    (sum)
}


fn calculate_manhattan_distance(target: i32) {
    let coord = calculate_coords(target);
    let dist = coord.0.abs() + coord.1.abs();
    println!("Manhattan distance is {}", dist);
}


fn calculate_coords(target: i32) -> (i32, i32){
    let mut x = 0;
    let mut y = 0;
    let mut i = 1;
    let mut incs = 1;
    let mut up_or_right = 1;

    while i < target {
        for j in 0..incs {
            if i == target {
                break;
            }
            i += 1;
            x += up_or_right;
        }

        for j in 0..incs {
            if i == target {
                break;
            }
            i += 1;
            y += up_or_right;
        }

        up_or_right = up_or_right * -1;


        incs += 1;


    }

    println!("number {} is in {},{}", i, x, y);

    (x,y)
}
