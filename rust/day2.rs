use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    const INPUT_FILE: &str = "../input2.txt";
    let mut count_1 = 0;
    let mut count_2 = 0;

    // Task 1
    for string_vector in create_vector(INPUT_FILE) {
        if parse_string(string_vector) {
            count_1 += 1;
        }
    }

    //Task 2
    for string_vector in create_vector(INPUT_FILE) {
        if parse_string_2(string_vector) {
            count_2 += 1;
        }
    }

    println!("Number of correct passwords: {}", count_1);
    println!("Number of correct passwords: {}", count_2);

}


fn parse_string(row: String) -> bool {
    let split = row.split_whitespace();
    let vec: Vec<&str> = split.collect();

    let range = &vec[0];
    let interval: Vec<i32> = range.split("-").map(|s| s.parse().unwrap()).collect();
    let search_string: &str = &vec[2];
    let lookup_character = &vec[1].chars().nth(0).unwrap();

    let mut count: i32 = 0;
    for char in search_string.chars() {
        if char == *lookup_character {
            count += 1
        }
    }

    let res = (count >= interval[0]) & (count <= interval[1]);
    res
}

fn parse_string_2(row: String) -> bool {
    let split = row.split_whitespace();
    let vec: Vec<&str> = split.collect();

    let range = &vec[0];
    let interval: Vec<i32> = range.split("-").map(|s| s.parse().unwrap()).collect();
    let lookup_character = &vec[1].chars().nth(0).unwrap();

    let first_character = &vec[2].chars().nth( (interval[0]-1) as usize ).unwrap();
    let second_character = &vec[2].chars().nth( (interval[1]-1) as usize ).unwrap();

    (first_character == lookup_character) ^ (second_character == lookup_character)
}

fn create_vector(input_file: &str) -> Vec<String> {
    let mut input_vector: Vec<String> = Vec::new(); 
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                input_vector.push( ip );
            }
        }
    }
    input_vector
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}