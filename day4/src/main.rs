use std::fs::File;
use std::io::{BufRead, BufReader};

fn overlapping(range1: Vec<&str>, range2: Vec<&str>) -> bool {
    let range1_start: i32 = range1[0].parse().unwrap();
    let range1_end: i32 = range1[1].parse().unwrap();

    let range2_start: i32 = range2[0].parse().unwrap();
    let range2_end: i32 = range2[1].parse().unwrap();

    if range1_start >= range2_start && range1_start <= range2_end {
        return true;
    }

    if range1_end >= range2_start && range1_end <= range2_end{
        return true;
    }

    if range2_start >= range1_start && range2_start <= range1_end {
        return true;
    }

    if range2_end >= range1_start && range2_end <= range1_end{
        return true;
    }

    return false;
}

fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();

    let mut count = 0;
    for line in lines {
        let mut delim = ',';
        
        let split_result: Vec<&str> = line.split(delim).collect();

        delim = '-';
        let elf1_range: Vec<&str> = split_result[0].split(delim).collect();
        let elf2_range: Vec<&str> = split_result[1].split(delim).collect();

        if overlapping(elf1_range, elf2_range) {
            println!("{} overlaps with {}", split_result[0], split_result[1]);
            count += 1;
        } 
        // println!("{:?}", elf1_range);
        // println!("{:?}", elf2_range);
    }

    println!("Total: {}", count);
}







// Part 1 Below


// use std::fs::File;
// use std::io::{BufRead, BufReader};
//
// fn contains_range(range1: Vec<&str>, range2: Vec<&str>) -> bool {
//     let range1_start: i32 = range1[0].parse().unwrap();
//     let range1_end: i32 = range1[1].parse().unwrap();
//
//     let range2_start: i32 = range2[0].parse().unwrap();
//     let range2_end: i32 = range2[1].parse().unwrap();
//
//     if range1_start >= range2_start && range1_end <= range2_end {
//         return true;
//     }
//
//     if range1_start <= range2_start && range1_end >= range2_end {
//         return true;
//     }
//
//     return false;
// }
//
// fn main() {
//     let file = File::open("input.txt").expect("Cannot open file");
//     let reader = BufReader::new(file);
//     let lines: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();
//
//     let mut count = 0;
//     for line in lines {
//         let mut delim = ',';
//         
//         let split_result: Vec<&str> = line.split(delim).collect();
//
//         delim = '-';
//         let elf1_range: Vec<&str> = split_result[0].split(delim).collect();
//         let elf2_range: Vec<&str> = split_result[1].split(delim).collect();
//
//         if contains_range(elf1_range, elf2_range) {
//             println!("{} contains {} (or vice versa)", split_result[0], split_result[1]);
//             count += 1;
//         } 
//         // println!("{:?}", elf1_range);
//         // println!("{:?}", elf2_range);
//     }
//
//     println!("Total: {}", count);
// }
