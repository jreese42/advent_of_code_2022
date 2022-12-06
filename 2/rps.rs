use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut score = 0;

    let stdin = io::stdin();
    //Rock:    A X      A beats C       A loses B
    //Paper:   B Y      B beats A       B loses C
    //Scissors C Z      C beats B       C loses A
    

    for line in stdin.lock().lines() {
        let line2 = line.unwrap();
        let strat_guide_line : Vec<_> = line2.split_whitespace().collect();
        
        let their_pick = strat_guide_line[0].chars().nth(0).unwrap();
        let mut my_pick = strat_guide_line[1].chars().nth(0).unwrap();

        //Lazy lookup to normalize XYZ to ABC
        if my_pick == 'X' { my_pick = 'A'; }
        else if my_pick == 'Y' { my_pick = 'B'; }
        else if my_pick == 'Z' { my_pick = 'C'; }

        //Score our selection A=1, B=2, C=3
        if my_pick == 'C' { score += 3; }
        else if my_pick == 'B' { score += 2; }
        else if my_pick == 'A' { score += 1; }

        //Score our outcomes
        //Ties
        if their_pick == my_pick 
        { 
            score += 3;
            println!("{} ties {}, score {}", my_pick, their_pick, score);
        }
        //Wins
        else if (their_pick == 'C' && my_pick == 'A')
                || (their_pick == 'A' && my_pick == 'B')
                || (their_pick == 'B' && my_pick == 'C')
        {
            score += 6;
            println!("{} beats {}, score {}", my_pick, their_pick, score)
        }
        else
        {
            println!("{} loses {}, score {}", my_pick, their_pick, score)
        }
    }

    //Print the final result
    println!("Final Score: {}", score);

    Ok(())
}