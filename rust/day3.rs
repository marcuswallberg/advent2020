use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    let tree_map: Vec<Vec<i32>> = parse_input("../input3.txt");

    //plot_path(&tree_map);

    let right_list: Vec<i32> = vec![1,3,5,7,1];
    let down_list: Vec<i32> = vec![1,1,1,1,2];


    let mut total_sum: i64= 1;
    for i in 0..right_list.len() {
        total_sum *= calculate_number_of_trees(&tree_map, &right_list[i], &down_list[i]) as i64;
    }

    println!("{}", total_sum);
}

fn calculate_number_of_trees(tree: &Vec<Vec<i32>>, right: &i32, down: &i32) -> i32 {
    let mut count = 0;
    let mut current_position = 0;
    for i in 0..tree.len() {
        if i % *down as usize == 0 {
            let width = tree[i].len();
            if tree[i][current_position] == 1 {
                count += 1;
            }
            current_position = (current_position + *right as usize) % width;
        }
    }
    count
}

fn parse_input(filename: &str) -> Vec<Vec<i32>> {
    let mut tree_map: Vec<Vec<i32>> = Vec::new();
    
    if let Ok(lines) = read_lines(filename) {
        for line in lines {

            let mut row: Vec<i32> = Vec::new();
            if let Ok(ip) = line {
                for char in ip.chars() {
                    if char == '#' {
                        row.push(1);
                    } else {
                        row.push(0);
                    }
                }
            }
            tree_map.push(row);
        }
    }
    tree_map
}

fn plot_path(tree: &Vec<Vec<i32>>) {
    for row in tree {
        for cell in row {
            let marker = if *cell == 1 {'#'} else {'.'};
            print!("{}",marker)
        }
        println!("");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}