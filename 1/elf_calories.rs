use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    // We don't need to keep track of all elves, only the top 3 elves
    let mut elf_tracker = [(0,0), (0,0), (0,0)];
    let mut current_elf_num = 1;
    let mut current_accum = 0;

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line2 = line.unwrap();
        println!("{}", line2);
        if line2.trim().is_empty() {
            sort_into_tracker(&mut elf_tracker, current_elf_num, current_accum);
            current_elf_num += 1;
            current_accum = 0;
        }
        else {
            //Process as int
            let val = line2.parse::<i32>().unwrap();
            current_accum += val;
        }

    }

    sort_into_tracker(&mut elf_tracker, current_elf_num, current_accum);

    //Print the final result
    println!("1st: Elf {} => {} calories", elf_tracker[0].0, elf_tracker[0].1);
    println!("2nd: Elf {} => {} calories", elf_tracker[1].0, elf_tracker[1].1);
    println!("3rd: Elf {} => {} calories", elf_tracker[2].0, elf_tracker[2].1);
    println!("Top 3 Total: {} calories", elf_tracker[0].1 + elf_tracker[1].1 + elf_tracker[2].1);

    Ok(())
}

fn sort_into_tracker(tracker : &mut [(i32,i32)], elf_num : i32, elf_calories : i32) {
    if elf_calories > tracker[0].1 { //gt 1st place, push back 1st to 2nd, 2nd to 3rd
        tracker[2].0 = tracker[1].0;
        tracker[2].1 = tracker[1].1;
        tracker[1].0 = tracker[0].0;
        tracker[1].1 = tracker[0].1;
        tracker[0].0 = elf_num;
        tracker[0].1 = elf_calories;
    }
    else if elf_calories > tracker[1].1 { //gt 2nd place, push 2nd to 3rd
        tracker[2].0 = tracker[1].0;
        tracker[2].1 = tracker[1].1;
        tracker[1].0 = elf_num;
        tracker[1].1 = elf_calories;
    }
    else if elf_calories > tracker[2].1 { //gt 3rd place
        tracker[2].0 = elf_num;
        tracker[2].1 = elf_calories;
    }
}
// newline means new elf
// compare if this is the new high
// otherwise, add to the working sum
