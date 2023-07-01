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
        'A' | 'X' => return Shape::Rock,
        'B' | 'Y' => return Shape::Paper,
        'C' | 'Z' => return Shape::Scissors,
        _ => panic!("unknown shape"),
    }
}

fn get_result(result: char) -> Outcome {
    match result {
        'X' => return Outcome::Lose,
        'Y' => return Outcome::Draw,
        'Z' => return Outcome::Win,
        _ => panic!("unknown result"),
    }
}

fn calculate_score(elf: Shape, outcome: Outcome) -> u32 {
    let mut score = 0u32;

    // match score
    match (elf, outcome) {
        (Shape::Rock, Outcome::Lose) => {
            score += Shape::Scissors as u32 + Outcome::Lose as u32;
        }
        (Shape::Rock, Outcome::Draw) => {
            score += Shape::Rock as u32 + Outcome::Draw as u32;
        }
        (Shape::Rock, Outcome::Win) => {
            score += Shape::Paper as u32 + Outcome::Win as u32;
        }

        (Shape::Paper, Outcome::Lose) => {
            score += Shape::Rock as u32 + Outcome::Lose as u32;
        }
        (Shape::Paper, Outcome::Draw) => {
            score += Shape::Paper as u32 + Outcome::Draw as u32;
        }
        (Shape::Paper, Outcome::Win) => {
            score += Shape::Scissors as u32 + Outcome::Win as u32;
        }

        (Shape::Scissors, Outcome::Lose) => {
            score += Shape::Paper as u32 + Outcome::Lose as u32;
        }
        (Shape::Scissors, Outcome::Draw) => {
            score += Shape::Scissors as u32 + Outcome::Draw as u32;
        }
        (Shape::Scissors, Outcome::Win) => {
            score += Shape::Rock as u32 + Outcome::Win as u32;
        }
    }

    score
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
