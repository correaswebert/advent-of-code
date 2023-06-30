use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut elves: Vec<u32> = Vec::new();
    let mut calories: u32 = 0;

    for line in contents {
        if line.is_empty() {
            elves.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    elves.sort();
    elves.reverse();

    println!("{}", elves[0] + elves[1] + elves[2]);
}

