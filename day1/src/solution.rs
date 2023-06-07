use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("aocday1.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut curr_cals = 0;
    let mut max_cals = 0;
    let mut top_three: Vec<i32> = Vec::new();

    for line in lines {
        if !line.is_empty() {
            curr_cals += line.parse::<i32>().unwrap();
        } else {
            if curr_cals > max_cals {
                top_three.push(curr_cals);
                if top_three.len() > 3 {
                    top_three.sort();
                    top_three.remove(0);
                }
                max_cals = curr_cals;
            }
            curr_cals = 0;
        }
    }

    println!("{:?}, {}", top_three, top_three.iter().sum::<i32>());
}
