use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get the filename from the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];

    // Open the file
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            return;
        }
    };

    // Create readers and vectors to hold the numbers
    let reader = BufReader::new(file);
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Split the line into two numbers
                let numbers: Vec<f64> = line
                    .split_whitespace()
                    .map(|s| s.parse::<f64>().unwrap())
                    .collect();

                // Check that there are exactly two numbers
                if numbers.len() != 2 {
                    eprintln!("Invalid line: {}", line);
                    continue;
                }

                // Add the numbers to the vectors
                col1.push(numbers[0]);
                col2.push(numbers[1]);
            }
            Err(err) => {
                eprintln!("Error reading line: {}", err);
            }
        }
    }

    // Print the vectors
    println!("Column 1: {:?}", col1);
    println!("Column 2: {:?}", col2);
}
