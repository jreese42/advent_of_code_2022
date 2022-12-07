use std::io;
use std::io::prelude::*;
use std::process;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut total_priority_part1 = 0;
    let mut total_priority_part2 = 0;
    let mut group_badges : Vec<char> = Vec::new();
    let mut group_count = 0;

    for line_opt in stdin.lock().lines() {
        let rucksack = line_opt.unwrap();

        //Part 1: Compare left half to right half to find 1 shared item

        let left_half = dedup_rucksack(&rucksack[..(rucksack.len() / 2)]);
        let right_half = dedup_rucksack(&rucksack[(rucksack.len() / 2)..]);

        let mut local_shared_count = 0;
        let mut local_total_priority = 0;

        //Iterate the items in the left half, score them if they appear in the right half
        for item in left_half.iter() {
            match right_half.iter().position(|c| c == item ) {
                Some(_x) => {
                    local_shared_count += 1;
                    local_total_priority += item_priority(item);
                },
                None => (),
            }
        }

        total_priority_part1 += local_total_priority;
        println!("{:?} and {:?} share {} items for {} total priority.", left_half, right_half, local_shared_count, local_total_priority);

        //Part 2: Compare each set of 3 lines to find 1 shared item
        //Dedup the whole line
        let mut deduped = dedup_rucksack(&rucksack);

        //If this is the first rucksack, store this whole rucksack, overwriting previous
        if group_count == 0 {
            group_badges = deduped.to_vec();
        }

        //else compare this line to the current known items, and only keep the common items
        else {
            reduce_rucksack(&mut group_badges, &mut deduped);
        }

        //after reducing the 3rd group, exactly 1 item should be left, so score it
        if group_count == 2 {
            if group_badges.len() > 1 {
                println!("Failed to find a group's badge item!");
                process::exit(1);
            }
            else {
                total_priority_part2 += item_priority(&group_badges[0]);
            }
        }

        // println!("Group {}: {:?}", group_count, group_badges);
        group_count = (group_count + 1) % 3;

    }

    println!("Total Priority Part 1: {}", total_priority_part1);
    println!("Total Priority Part 2: {}", total_priority_part2);

    Ok(())
}

fn item_priority(item : &char) -> i32 {
    //a=1, z=26, A=27, Z=52
    //rust doesn't assume ascii formatted strings, so ascii math isn't useful here.
    //let's provide an ordered map of values we can lookup instead.
    let value_map = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    value_map.chars().position(|c| c == *item ).unwrap() as i32 + 1
}

fn dedup_rucksack(rucksack : &str) -> Vec<char> {
    //sort the characters in the string
    let mut items: Vec<char> = rucksack.chars().collect();
    items.sort_by(|a, b| b.cmp(a));
    items.dedup();
    items
}

fn reduce_rucksack(known_items : &mut Vec<char>, deduped_rucksack : &mut Vec<char>) {
    known_items.append(deduped_rucksack); //shtick em together
    let known_items_2 : Vec<char> = known_items.to_vec(); //being lazy about reference management so I can do the retain trick below
    
    //Remove any characters that are singular - they can't be it

    known_items.retain(|&c| &known_items_2.iter().filter(|&n| *n == c).count() > &mut 1);

    //Finally, dedup the list for the final output
    //If this was the 3rd group, hopefully exactly 1 is left
    known_items.sort_by(|a, b| b.cmp(a));
    known_items.dedup();
}