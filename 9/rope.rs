use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let rope_length = 10;
    let mut rope : Vec<(i32, i32)> = vec![(0,0); rope_length];

    let mut visited : HashSet<(i32, i32)> = HashSet::new();

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap();
        println!("New Instruction: {}", line);
        let tokens : Vec<&str> = line.split(" ").collect();

        let mut head_move = (0,0);
        match tokens[0] {
            "L" => {
                head_move = (-1,0);
            },
            "R" => {
                head_move = (1,0);
            },
            "U" => {
                head_move = (0,1);
            },
            "D" => {
                head_move = (0,-1);
            },
            _ => {

            }
        }

        for _i in 0..tokens[1].parse::<i32>().unwrap() {

            //Move head according to instruction
            rope[0].0 += head_move.0;
            rope[0].1 += head_move.1;

            //move each rope segment beyond head according to rules
            for segment in 1..rope_length {
                let m = calc_move(&rope[segment-1], &rope[segment]);
                rope[segment].0 += m.0;
                rope[segment].1 += m.1;
            }

            visited.insert(rope[rope_length - 1]);

            print_board(&rope, &visited);
        }
    }

    println!("Visited {} locations total.", visited.len());

    Ok(())
}

fn calc_move(front : &(i32, i32), rear : &(i32, i32)) -> (i32, i32) {
    let dist = (front.0 - rear.0, front.1 - rear.1);
    let mut ret = (0,0);
    if (dist.0.abs() == 2 && dist.1 == 0) || (dist.0 == 0 && dist.1.abs() == 2) {
        //Cardinal Move by 1
        ret.0 = dist.0 / 2;
        ret.1 = dist.1 / 2;
    }
    //else, rear steps diagonally by 1 in direction is towards head
    else if (front.0 != rear.0) && (front.1 != rear.1) && (dist.0.abs() > 1 || dist.1.abs() > 1) {
        ret.0 = dist.0 / dist.0.abs();
        ret.1 = dist.1 / dist.1.abs();
    }

    ret
}

fn print_board(rope : &Vec<(i32, i32)>, visited : &HashSet<(i32, i32)>) {
    for y in (0..5).rev() {
        for x in 0..6 {
            if rope[0] == (x,y) {
                print!("H");
            }
            else if rope.contains(&(x,y)) {
                print!("{}", rope.iter().position(|&c| c == (x,y)).unwrap());
            }
            else if visited.contains(&(x,y)) {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");

    // println!("Head: {},{}", head.0, head.1);
    // println!("Tail: {},{}", tail.0, tail.1);
    // println!("");

    io::stdout().flush().unwrap();
}