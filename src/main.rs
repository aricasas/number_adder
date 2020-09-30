#![warn(clippy::all)]

use rand::rngs;
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::cmp;
use std::fs;
use std::thread;
use std::time::Instant;

fn add_numbers_not_threaded(list: &[u32]) -> u64 {
    list.iter()
        .map(
            |number| u64::from(*number), // Convert numbers to u64
        )
        .sum()
}

fn add_numbers_manual_threads(list: &[u32], max_threads: u32) -> u64 {
    if max_threads == 0 {
        add_numbers_not_threaded(list)
    } else {
        // If it's not zero, it must be a positive number

        let list_length = list.len();
        let thread_count = cmp::min(list_length / 100_000, max_threads as usize);

        let split_data = if list_length % thread_count != 0 {
            // If it can't divide all the numbers into equal chunks,
            list.chunks((list_length / thread_count) + 1)
        } else {
            list.chunks(list_length / thread_count)
        };

        let mut children = vec![];

        for chunk in split_data {
            let clone = Vec::from(chunk);
            children.push(thread::spawn(move || add_numbers_not_threaded(&clone)))
        }

        let mut result = 0;
        for child in children {
            result += child.join().unwrap();
        }
        result
    }
}

fn add_numbers_rayon_threads(list: &[u32]) -> u64 {
    list.par_iter()
        .map(|number| u64::from(*number))
        .sum::<u64>()
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
    const RANDOM_NUMBER_COUNT: usize = 300_000_000;

    // Now generate our own numbers
    let range = rand::distributions::Uniform::new_inclusive(1, 1_000_000);

    let now = Instant::now();
    let numbers = generate_numbers(RANDOM_NUMBER_COUNT, range);
    println!(
        "Generating {} random numbers took: {}s\n_____",
        RANDOM_NUMBER_COUNT,
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );

    let now = Instant::now();
    println!("Sum is: {}", add_numbers_manual_threads(&numbers, 50));
    println!(
        "Adding all the numbers using the threaded function took: {}s\n_____",
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );

    let now = Instant::now();
    println!("Sum is: {}", add_numbers_not_threaded(&numbers));
    println!(
        "Adding all the numbers took: {}s\n_____",
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );

    let now = Instant::now();
    println!("Sum is: {}", add_numbers_rayon_threads(&numbers));
    println!(
        "Adding all the numbers using rayon threads took: {}s\n_____",
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );
}
