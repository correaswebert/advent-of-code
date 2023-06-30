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

    let mut max_calories: u32 = 0;
    let mut calories: u32 = 0;

    for line in contents {
        if line.is_empty() {
            if calories > max_calories {
                max_calories = calories;
            }
            calories = 0;
        } else {
            calories += line.parse::<u32>().unwrap();
        }
    }

    println!("{}", max_calories);
}

