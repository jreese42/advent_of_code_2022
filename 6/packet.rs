use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap(); //this one is just one line
        
        let chars : Vec<char> = line.chars().collect();

        //with that one line, find the marker position if the marker len is any given length.
        //Part 1: marker_len 4
        //Part 2: marker_len 14
        for marker_len in 4..15 {
            for i in marker_len..chars.len() {
                let slice = &mut chars[i-marker_len+1..i+1].to_vec();
    
                slice.sort_by(|a, b| b.cmp(a));
                slice.dedup();
                if slice.len() == marker_len {
                    println!("Found Packet Marker of len {} at position {}", marker_len, i+1);
                    break;
                }
            }
        }
    }

    Ok(())
}