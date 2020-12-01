use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec::Vec;

fn main() {
    const TARGET: u32 = 2020;

    // Check if the file actually extists before running script
    if let Ok(lines) = read_lines("../input1.txt") {

        let mut input_vector: Vec<u32> = Vec::new();

        // Reading each line and pushing it to a vector
        for line in lines {
            if let Ok(ip) = line {
                let int_value: u32 = ip.parse::<u32>().unwrap();
                input_vector.push( int_value );
            }
        }

        // For-looping all the number 3 times, avoiding multiple tries
        for i in 0..input_vector.len() {
            for j in i+1..input_vector.len() {
                for k in j+1..input_vector.len() {
                    if input_vector[i] + input_vector[j] + input_vector[k] == TARGET {
                        println!("The sum is: {}", input_vector[i] * input_vector[j] * input_vector[k] )
                    }
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}