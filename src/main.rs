use std::fs;
use std::time::Instant;

fn add_numbers(vector: &Vec<u32>) -> u64 {
    let mut x = 0;
    for number in vector.iter() {
        x += *number as u64;
    }
    x
}

fn read_numbers_from_file(filename: &str) -> Vec<u32> {
    let numbers = fs::read_to_string(filename).expect("Couldn't read file");
    numbers
        .lines() // Split on newlines
        .map(|number| number.parse::<u32>().unwrap()) // Make the Iterator parse the strings into u64
        .collect() // Make a Vector from the Iterator
}

fn main() {
    let now = Instant::now();
    let numbers = read_numbers_from_file("numbers/num2.txt");
    println!(
        "Getting a Vector from the file took: {}μs",
        now.elapsed().as_micros()
    );

    let now = Instant::now();
    println!("Sum is: {}", add_numbers(&numbers));
    println!(
        "Adding all the numbers took: {}μs",
        now.elapsed().as_micros()
    );
}
