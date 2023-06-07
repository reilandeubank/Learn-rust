use std::fs::File;
use std::io::{BufRead, BufReader};


fn get_priority(c: char) -> Option<u8> {
    let unicode_val = c as u32;

    if unicode_val >= 97 && unicode_val <= 122 {
        Some((unicode_val - 96) as u8)
    }
    else if unicode_val >= 65 && unicode_val <= 90 {
        Some((unicode_val - 38) as u8)
    }
    else {
        None
    }
}


fn main() {
    let file = File::open("input.txt").expect("Cannot open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut score: u32 = 0;
    let mut group: [String; 3] = [String::new(), String::new(), String::new()];
    let mut iteration = 0;

    for line in lines {
        group[iteration] = line;
        iteration += 1;
        if iteration == 3 {
            'inner_loop: for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    if let Some(priority) = get_priority(c) {
                        score += priority as u32;
                    }
                    break 'inner_loop;
                }
            }
            iteration = 0;
        }
        
    }
    println!("Score: {}", score);
}





// use std::fs::File;
// use std::io::{BufRead, BufReader};
//
//
// fn get_priority(c: char) -> Option<u8> {
//     let unicode_val = c as u32;
//
//     if unicode_val >= 97 && unicode_val <= 122 {
//         Some((unicode_val - 96) as u8)
//     }
//     else if unicode_val >= 65 && unicode_val <= 90 {
//         Some((unicode_val - 38) as u8)
//     }
//     else {
//         None
//     }
// }
//
//
// fn main() {
//     let file = File::open("input.txt").expect("Cannot open file");
//     let reader = BufReader::new(file);
//     let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
//
//     let mut score: u32 = 0;
//
//     for line in lines {
//         let middle = line.len() / 2;
//         let (first, second) = line.split_at(middle);
//
//         for c in first.chars() {
//             if second.contains(c) {
//                 // println!("Matching char is: {}", c);
//                 if let Some(priority) = get_priority(c) {
//                     score += priority as u32;
//                 }
//                 break;
//             }
//         }
//     }
//     println!("Score: {}", score);
// }
