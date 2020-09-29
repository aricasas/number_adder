#![warn(clippy::all)]

use rand::rngs;
use rand::Rng;
use rand::SeedableRng;
use std::fs;
use std::time::Instant;

fn add_numbers(list: &[u32]) -> u64 {
    list.iter()
        .map(
            |number| u64::from(*number), // Convert numbers to u64
        )
        .sum()
}

fn _read_numbers_from_file(filename: &str) -> Vec<u32> {
    let numbers = fs::read_to_string(filename).expect("Couldn't read file");
    numbers
        .lines() // Split on newlines, returns an iterator
        .map(|number| number.parse::<u32>().unwrap()) // Make the iterator parse the strings into u32
        .collect() // Make a Vector from the Iterator
}

fn generate_numbers(count: usize, range: rand::distributions::Uniform<u32>) -> Vec<u32> {
    let mut rng = rngs::SmallRng::from_entropy();
    let mut numbers = Vec::with_capacity(count);
    for _ in 0..count {
        numbers.push(rng.sample(&range))
    }

    numbers
}

fn main() {
    const RANDOM_NUMBER_COUNT: usize = 100_000_000;

    // Now generate our own numbers
    let range = rand::distributions::Uniform::new_inclusive(1, 1_000_000);

    let now = Instant::now();
    let numbers = generate_numbers(RANDOM_NUMBER_COUNT, range);
    println!(
        "Generating {} random numbers took: {}s",
        RANDOM_NUMBER_COUNT,
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );

    let now = Instant::now();
    println!("Sum is: {}", add_numbers(&numbers));
    println!(
        "Adding all the numbers took: {}s",
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );
}
