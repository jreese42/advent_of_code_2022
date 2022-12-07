use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut file_sizes : HashMap<PathBuf, i32> = HashMap::new();
    let mut paths_to_check : Vec<PathBuf> = Vec::new();
    let mut cwd = PathBuf::new();
    let mut best_dir = (PathBuf::new(), 0);
    cwd.push("/");
    let mut score = 0;

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap();
        let tokens : Vec<&str> = line.split(" ").collect();
        // println!("{:?}", tokens);
        match tokens[0] {
            "$" => {
                //command string
                match tokens[1] {
                    "cd" => {
                        match tokens[2] {
                            "/" => {
                                //reset cwd
                                cwd.clear();
                                cwd.push("/");
                            },
                            ".." => {
                                //pop from end of cwd
                                cwd.pop();
                                // println!("cd .., new dir {}", cwd.display());
                            },
                            _ => {
                                //push to end of cwd
                                cwd.push(tokens[2]);
                                // println!("cd {}, new dir {}", tokens[2], cwd.display());
                            }
                        }
                    },
                    "ls" => {
                        //ls commands.  ignore.
                    },
                    _ => {

                    }
                }

            },
            "dir" => {
                //dir entry. ignore.
            }
            _ => {
                //file size entry
                //push this final path to sum up later
                cwd.push(tokens[1]);
                file_sizes.insert(cwd.clone(), tokens[0].parse::<i32>().unwrap());
                println!("file {} size {}", cwd.display(), tokens[0]);
                cwd.pop();
            }
        };
    }

    //So now we have a list of every absolute file path with a size

    //get this of list to check by looking at every file we know and getting the parent tree all the way up
    //this is naieve, we end up with lots of duplicates but whatever, it works well enough
    for pair in file_sizes.iter() {
        let mut path = pair.0.clone();
        while let Some(_x) = path.parent()  {
            path.pop();
            let new_path = path.clone();
            if !paths_to_check.contains(&new_path)
            {
                paths_to_check.push(new_path);
            }
        }
    }

    //Part 2: find the smallest directory that frees up 30000000 of 70000000
    let space_used = file_sizes.iter().fold(0, |acc, x| acc + x.1 );
    let space_needed = 30000000 - (70000000 - space_used);

    //To get the size of any dir, sum up anything that starts with the absolute path to that dir
    //Score anything <= 100000 for part 1
    for path in paths_to_check.iter() {
        let sum = file_sizes.iter().fold(0, |acc, x| if x.0.starts_with(path) { acc + x.1 } else { acc });
        println!("Checking {}, Size is {}. Best dir size so far is {}", path.display(), sum, best_dir.1);
        if sum <= 100000 {
            score += sum;
        }

        if sum >= space_needed && (sum < best_dir.1 || best_dir.1 == 0) {
            best_dir = (path.clone(), sum);
        }
    }

    println!("Total Score: {}", score);
    println!("Used Space is {}, Needed space is {}", space_used, space_needed);
    println!("Best Dir is {} which frees {}", best_dir.0.display(), best_dir.1);

    Ok(())
}