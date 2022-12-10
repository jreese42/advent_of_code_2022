use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut cycle_num : i32 = 0;
    let mut score = 0;
    let mut x = 1;

    let mut isr_q : Vec<i32> = Vec::new();

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap();
        let tokens : Vec<&str> = line.split(" ").collect();
        
        match tokens[0] {
            "addx" => {
                isr_q.push(0);
                isr_q.push(tokens[1].parse::<i32>().unwrap());
            },
            "noop" => {
                isr_q.push(0);
            },
            _ => {

            }
        }

        //Run "cpu"
        for n in isr_q.drain(0..) {
            cycle_num += 1;

            if (cycle_num-20) % 40 == 0 {
                score += cycle_num * x;
            }

            //Part 2: CRT Printing
            if (cycle_num%40)-1 >= x-1 && (cycle_num%40)-1 <= x+1 {
                print!("#");
            }
            else {
                print!{"."};
            }

            if cycle_num % 40 == 0 {
                print!("\n");
                io::stdout().flush().unwrap();
            }

            x += n;
        }

    }

    println!("Part 1 Final Score {}", score);

    Ok(())
}