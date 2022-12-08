use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut forest : Vec<Vec<u32>> = Vec::new();
    let mut row_idx : usize = 0;
    let mut visible_trees = 0;
    let mut high_scenic_score = 0;

    for line_opt in stdin.lock().lines() {
        let line = line_opt.unwrap();
        forest.push(Vec::new());
        for tree in line.chars() {
            forest[row_idx].push(tree.to_digit(10).unwrap());
        }
        row_idx += 1;
    }

    //Check each of 4 directions for visibility
    let mut x_coord = 0;
    let mut y_coord = 0;
    for row in forest.iter() {
        y_coord += 1;
        x_coord = 0;

        'nexttree: for tree_ht in row.iter() {
            x_coord += 1;

            //Border trees are always visible
            if x_coord == 1 || x_coord == row.len() || y_coord == 1 || y_coord == forest.len() {
                println!("Tree at {} {} is on the perimeter!", x_coord, y_coord);
                visible_trees += 1;
                continue 'nexttree;
            }

            //Check W
            let mut visible = true;
            'cmp_W: for x in x_coord..row.len() {
                if row[x] >= *tree_ht {
                    visible = false;
                    break 'cmp_W;
                }
            }
            if visible {
                println!("Tree at {} {} is visible from W!", x_coord, y_coord);
                visible_trees += 1;
                continue 'nexttree;
            }

            //Check E
            visible = true;
            'cmp_E: for x in 0..x_coord-1 {
                if row[x] >= *tree_ht {
                    visible = false;
                    break 'cmp_E;
                }
            }
            if visible { 
                println!("Tree at {} {} is visible from E!", x_coord, y_coord);
                visible_trees += 1;
                continue 'nexttree;
            }

            //Check S
            visible = true;
            'cmp_S: for y in y_coord..forest.len() {
                if forest[y][x_coord-1] >= *tree_ht {
                    visible = false;
                    break 'cmp_S;
                }
            }
            if visible { 
                println!("Tree at {} {} is visible from S!", x_coord, y_coord);
                visible_trees += 1;
                continue 'nexttree;
            }

            //Check N
            visible = true;
            'cmp_N: for y in 0..y_coord-1 {
                if forest[y][x_coord-1] >= *tree_ht {
                    visible = false;
                    break 'cmp_N;
                }
            }
            if visible { 
                println!("Tree at {} {} is visible from N!", x_coord, y_coord);
                visible_trees += 1;
                continue 'nexttree;
            }
        }
    }


    //Part 2: Perform a similar search, but instead of breaking, count scenic score in each dir
    x_coord = 0;
    y_coord = 0;
    for row in forest.iter() {
        y_coord += 1;
        x_coord = 0;

        for tree_ht in row.iter() {
            x_coord += 1;
            let mut scenic_score = 1;
            let mut tree_cnt = 0;

            //Border trees are terrible places for treehouses
            if x_coord == 1 || x_coord == row.len() || y_coord == 1 || y_coord == forest.len() {
                continue;
            }

            //Score W
            for x in x_coord..row.len() {
                tree_cnt += 1;
                if row[x] >= *tree_ht {
                    break;
                }
            }
            scenic_score *= tree_cnt;

            //Check E
            tree_cnt = 0;
            for x in (0..x_coord-1).rev() {
                tree_cnt += 1;
                if row[x] >= *tree_ht {
                    break;
                }
            }
            scenic_score *= tree_cnt;

            //Check S
            tree_cnt = 0;
            for y in y_coord..forest.len() {
                tree_cnt += 1;
                if forest[y][x_coord-1] >= *tree_ht {
                    break;
                }
            }
            scenic_score *= tree_cnt;

            //Check N
            tree_cnt = 0;
            for y in (0..y_coord-1).rev() {
                tree_cnt += 1;
                if forest[y][x_coord-1] >= *tree_ht {
                    break;
                }
            }
            scenic_score *= tree_cnt;

            if scenic_score > high_scenic_score {
                high_scenic_score = scenic_score;
            }
        }
    }

    for row in forest.iter() {
        println!("{:?}", row);
    }

    println!("Count of Visible Trees is {}", visible_trees);
    println!("Highest Scenic Score is {}", high_scenic_score);

    Ok(())
}