#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::correctness,
    clippy::complexity,
    clippy::perf
)]
#![allow(clippy::cast_precision_loss)]

use rand::rngs;
use rand::Rng;
use rand::SeedableRng;
use rayon::prelude::*;
use std::cmp;
use std::fs;
use std::thread;
use std::time::Instant;

fn add_numbers_not_threaded(list: &[u32]) -> u64 {
    // Iterates through all of the numbers and adds them
    list.iter().map(|number| u64::from(*number)).sum()
}

fn add_numbers_manual_threads(list: &[u32], max_threads: u32) -> u64 {
    if max_threads == 0 {
        // If max_threads is 0, don't use threads
        add_numbers_not_threaded(list)
    } else {
        // If it's not zero, it must be a positive number

        let list_length = list.len();
        let thread_count = cmp::min(list_length / 100_000, max_threads as usize);

        let split_data = if list_length % thread_count == 0 {
            list.chunks(list_length / thread_count)
        } else {
            // If it can't divide all the numbers into equal chunks,
            // add one to ths size we want the chunks to be.
            // This makes it so the remainder
            list.chunks((list_length / thread_count) + 1)
        };

        let mut children = vec![];

        for chunk in split_data {
            // Rust makes sure you can't use an invalid reference
            // so it deosn't allow you to borrow the chunk directly
            // because there's a chance that the list the chunk is
            // from gets freed. So we have to clone the data.
            // That makes this function very slow
            let clone = Vec::from(chunk);
            children.push(thread::spawn(move || add_numbers_not_threaded(&clone)))
        }

        let mut result = 0;
        for child in children {
            // Add all of the intermediate results we get
            // from the other threads
            result += child.join().unwrap();
        }
        result
    }
}

fn add_numbers_rayon_threads(list: &[u32]) -> u64 {
    // Wow. Using rayon sure made it a whole lot easier.
    // And it's also a lot faster
    list.par_iter()
        .map(|number| u64::from(*number))
        .sum::<u64>()
}

fn _read_numbers_from_file(filename: &str) -> Vec<u32> {
    // Reading from the file is a lot slower than generating
    // our own random numbers at runtime
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
        // Add a number within the range to the vector
        numbers.push(rng.sample(&range))
    }

    numbers
}

fn main() {
    const RANDOM_NUMBER_COUNT: usize = 300_000_000;

    // The range in which numbers are allowed
    // All the numbers we generate will be betweem 1 and 1,000,000 inclusive
    let range = rand::distributions::Uniform::new_inclusive(1, 1_000_000);

    // Let's see how long it takes to generate numbers
    let now = Instant::now();
    let numbers1 = generate_numbers(RANDOM_NUMBER_COUNT, range);
    println!(
        "Generating {} random numbers took: {}s\n_____",
        RANDOM_NUMBER_COUNT,
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );

    // Now let's see how long it takes add all of them
    let now = Instant::now();
    println!("Sum is: {}", add_numbers_not_threaded(&numbers1));
    println!(
        "Adding all the numbers took: {}s\n_____",
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );

    // Now let's see how long it takes add all of them using
    // the fuction that uses threads that I implemented
    // In release mode, this one is the slowest because
    // it has to copy a lot of data
    // But without optimizations enabled, this one beats the first one
    let now = Instant::now();
    println!("Sum is: {}", add_numbers_manual_threads(&numbers1, 50));
    println!(
        "Adding all the numbers using the threaded function took: {}s\n_____",
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );

    // Now let's see how long it takes add all of them using
    // the fuction that uses rayon (this one is the fastest)
    let now = Instant::now();
    println!("Sum is: {}", add_numbers_rayon_threads(&numbers1));
    println!(
        "Adding all the numbers using rayon threads took: {}s\n_____",
        now.elapsed().as_micros() as f64 / 1_000_000.0
    );
}
