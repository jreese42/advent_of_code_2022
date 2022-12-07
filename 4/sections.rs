use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut count = 0;
    let mut count2 = 0;
    let mut range1 : std::ops::Range<i32>;
    let mut range2 : std::ops::Range<i32>;

    for line_opt in stdin.lock().lines() {
        let assignments = line_opt.unwrap();
        let assignments_split: Vec<&str> = assignments.split(",").collect();

        let elf1 : Vec<&str> = assignments_split[0].split("-").collect();
        let elf2 : Vec<&str> = assignments_split[1].split("-").collect();
        range1 = elf1[0].parse::<i32>().unwrap()..elf1[1].parse::<i32>().unwrap()+1;
        range2 = elf2[0].parse::<i32>().unwrap()..elf2[1].parse::<i32>().unwrap()+1;
        println!("{:?}  {:?}", range1, range2);

        //Part1: Score if either range is fully contained by the other
        if (range1.start >= range2.start && range1.end <= range2.end)
            || (range2.start >= range1.start && range2.end <= range1.end) {
            count += 1;
        }

        //Part 2: Score if any overlap exists
        if range1.contains(&range2.start) || range1.contains(&(&range2.end-1))
            || range2.contains(&range1.start) || range2.contains(&(&range1.end-1)) {
            count2 += 1;
        }

    }

    println!("Full Overlap Total Count: {}", count);
    println!("Partial Overlap Total Count: {}", count2);

    Ok(())
}