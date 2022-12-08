use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;


//Create x queues for each stack
//iterating from top, read line len to get stack count then check each letter spot
//on first readthrough, push to front of queues
//upon "move" instruction, begin pop and push procedure

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut stacks_part1 : Vec<VecDeque<char>> = Vec::new();
    let mut stacks_part2 : Vec<VecDeque<char>> = Vec::new();

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap();

        let num_stacks = (line.len() + 1) / 4;

        if stacks_part1.len() == 0 {
            //First line, construct the stacks
            for _c in 0..num_stacks {
                stacks_part1.push(VecDeque::new());
                stacks_part2.push(VecDeque::new());
            }
        }

        if line.starts_with("move") {
            //This is a move instruction, so do it
            //"move C from X to Y"
            let tokens : Vec<&str> = line.split(" ").collect();
            let from_idx = tokens[3].parse::<i32>().unwrap() - 1;
            let to_idx = tokens[5].parse::<i32>().unwrap() - 1;

            for _count in 0..tokens[1].parse::<i32>().unwrap() {
                let c = stacks_part1[from_idx as usize].pop_back().unwrap();
                stacks_part1[to_idx as usize].push_back(c);
            }

            //For Part 2, run the same operation, but pass through a temp queue first
            let mut temp_q : Vec<char> = Vec::new();
            for _count in 0..tokens[1].parse::<i32>().unwrap() {
                temp_q.push(stacks_part2[from_idx as usize].pop_back().unwrap());
            }
            for _count in 0..temp_q.len() {
                stacks_part2[to_idx as usize].push_back(temp_q.pop().unwrap());
            }
        }
        else {
            //This is the list of stacks_part1 at the start of input.  Push it onto the queues.
            println!("Num Stacks is {}", num_stacks);

            for x in 0..num_stacks {
                let c : char = line.chars().nth(1+(4*x)).unwrap();
                if c != ' ' {
                    println!("Crate {}", c);
                    stacks_part1[x].push_front(c);
                    stacks_part2[x].push_front(c);
                }
            }
        }
    }

    println!("Final Crates on Top of Stacks, Part 1");
    for x in 0..stacks_part1.len() {
        print!("{}", stacks_part1[x].pop_back().unwrap());
    }
    print!("\n");
    io::stdout().flush().unwrap();

    println!("Final Crates on Top of Stacks, Part 2");
    for x in 0..stacks_part1.len() {
        print!("{}", stacks_part2[x].pop_back().unwrap());
    }
    print!("\n");
    io::stdout().flush().unwrap();


    Ok(())
}