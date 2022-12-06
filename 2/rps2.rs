use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut score = 0;

    let stdin = io::stdin();
    // X means play to lose
    // Y means play to draw
    // Z means play to win
    

    for line in stdin.lock().lines() {
        let line2 = line.unwrap();
        let strat_guide_line : Vec<_> = line2.split_whitespace().collect();
        
        let their_pick = strat_guide_line[0].chars().nth(0).unwrap();
        let my_strat = strat_guide_line[1].chars().nth(0).unwrap();

        match my_strat {
            'Z' => {
                //Play to win
                if their_pick == 'A' { score += 8; } //my pick is B, score = 6 + 2
                if their_pick == 'B' { score += 9; } //my pick is C, score = 6 + 3
                if their_pick == 'C' { score += 7; } //my pick is A, score = 6 + 1

            }
            'Y' => {
                //Play to tie
                if their_pick == 'A' { score += 4; } //my pick is A, score = 3 + 1
                if their_pick == 'B' { score += 5; } //my pick is B, score = 3 + 2
                if their_pick == 'C' { score += 6; } //my pick is C, score = 3 + 3
                
            }
            'X' => {
                //Play to lose
                if their_pick == 'A' { score += 3; } //my pick is C, score = 0 + 3
                if their_pick == 'B' { score += 1; } //my pick is A, score = 0 + 1
                if their_pick == 'C' { score += 2; } //my pick is B, score = 0 + 2
            }
            _ => {
                println!("Bad pick!");
            }
        }
    }

    //Print the final result
    println!("Final Score: {}", score);

    Ok(())
}