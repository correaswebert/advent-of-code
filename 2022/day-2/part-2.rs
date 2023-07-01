use std::env;
use std::fs;

enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

fn get_shape(shape_char: char) -> Shape {
    match shape_char {
        'A' | 'X' => Shape::Rock,
        'B' | 'Y' => Shape::Paper,
        'C' | 'Z' => Shape::Scissors,
        _ => panic!("unknown shape"),
    }
}

fn get_result(result: char) -> Outcome {
    match result {
        'X' => Outcome::Lose,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!("unknown result"),
    }
}

fn calculate_score(elf: Shape, outcome: Outcome) -> u32 {
    match (elf, outcome) {
        (Shape::Rock, Outcome::Lose) => {
            Shape::Scissors as u32 + Outcome::Lose as u32
        }
        (Shape::Rock, Outcome::Draw) => {
            Shape::Rock as u32 + Outcome::Draw as u32
        }
        (Shape::Rock, Outcome::Win) => {
            Shape::Paper as u32 + Outcome::Win as u32
        }

        (Shape::Paper, Outcome::Lose) => {
            Shape::Rock as u32 + Outcome::Lose as u32
        }
        (Shape::Paper, Outcome::Draw) => {
            Shape::Paper as u32 + Outcome::Draw as u32
        }
        (Shape::Paper, Outcome::Win) => {
            Shape::Scissors as u32 + Outcome::Win as u32
        }

        (Shape::Scissors, Outcome::Lose) => {
            Shape::Paper as u32 + Outcome::Lose as u32
        }
        (Shape::Scissors, Outcome::Draw) => {
            Shape::Scissors as u32 + Outcome::Draw as u32
        }
        (Shape::Scissors, Outcome::Win) => {
            Shape::Rock as u32 + Outcome::Win as u32
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut score = 0;

    for line in contents {
        let elf_char = line.chars().nth(0).unwrap();
        let outcome_char = line.chars().nth(2).unwrap();

        let elf = get_shape(elf_char);
        let outcome = get_result(outcome_char);

        score += calculate_score(elf, outcome);
    }

    println!("{score}");
}
