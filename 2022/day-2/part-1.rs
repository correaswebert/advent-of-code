use std::env;
use std::fs;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

fn get_shape(shape_char: char) -> Shape {
    match shape_char {
        'A' | 'X' => return Shape::Rock,
        'B' | 'Y' => return Shape::Paper,
        'C' | 'Z' => return Shape::Scissors,
        _ => panic!("unknown shape"),
    }
}

fn calculate_score(elf: Shape, me: Shape) -> u32 {
    let mut score = 0u32;

    // shape score
    match me {
        Shape::Rock => score += 1,
        Shape::Paper => score += 2,
        Shape::Scissors => score += 3,
    }

    // match score
    match (elf, me) {
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => score += 0,

        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => score += 3,

        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => score += 6,
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
        let me_char = line.chars().nth(2).unwrap();

        let elf = get_shape(elf_char);
        let me = get_shape(me_char);

        score += calculate_score(elf, me);
    }

    println!("{score}");
}
