use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut score = 0i32;
    let opp_moves = ['A', 'B', 'C'];
    // [rock, paper, scissors]

    for line in lines {
        let opponent_move = line.chars().nth(0).unwrap();
        let your_move = line.chars().nth(2).unwrap();

        //scoring outcome
        let opp_index = opp_moves.iter().position(|&r| r == opponent_move).unwrap() as i32;
        let mut your_index = 0i32;
        let _ = your_index;
        
        if your_move == 'X' {
            // println!("X");
            your_index = opp_index - 1;
            if your_index < 0 {
                your_index = 2;
            }
        }
        else if your_move == 'Y' {
            // println!("Y");
            score += 3;
            your_index = opp_index;
        }
        else {
            // println!("Z");
            score += 6;
            your_index = (opp_index + 1) % 3;
        }
        
        score += your_index + 1;
    }
    println!("score: {}", score);
}



// THIS IS PART 1, PART 2 CODE ABOVE
// use std::fs::File;
// use std::io::{BufRead, BufReader};
//
// fn main() {
//     let file = File::open("input.txt").expect("Failed to open file");
//     let reader = BufReader::new(file);
//     let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
//
//     let mut score = 0;
//     let opp_moves = ['A', 'B', 'C'];
//     let your_moves = ['X', 'Y', 'Z'];
//     // [rock, paper, scissors]
//
//     for line in lines {
//         let opponent_move = line.chars().nth(0).unwrap();
//         let your_move = line.chars().nth(2).unwrap();
//
//         //scoring outcome
//         let opp_index = opp_moves.iter().position(|&r| r == opponent_move).unwrap();
//         let your_index = your_moves.iter().position(|&r| r == your_move).unwrap();
//         
//         if opp_index == your_index {
//             score += 3;
//         }
//         else if (opp_index + 1) % 3 == your_index {
//             score += 6;
//         }
//         
//         // score from your move 
//         score += your_index + 1;
//     }
//     println!("score: {}", score);
// }
