use std::env;
use std::fs;

fn get_priority(item: char) -> u32 {
    match item {
        'a'..='z' => return item as u32 - 'a' as u32,
        'A'..='Z' => return item as u32 - 'A' as u32 + 26,
        _ => return 0,
    }
}

fn get_common_item_priority(rucksack: String) -> u64 {
    let len_compartment = rucksack.len() / 2;
    let compartment_1 = &rucksack[..len_compartment];
    let compartment_2 = &rucksack[len_compartment..];

    let mut items_1: u64 = 0;
    let mut items_2: u64 = 0;

    for item in compartment_1.chars() {
        let priority = get_priority(item);
        items_1 |= 1 << priority;
    }

    for item in compartment_2.chars() {
        let priority = get_priority(item);
        items_2 |= 1 << priority;
    }

    let mut common_item = items_1 & items_2;
    let mut common_item_priority: u64 = 0;

    while common_item > 0 {
        common_item_priority += 1;
        common_item >>= 1;
    }

    common_item_priority
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    
    let mut priority_sum: u64 = 0;

    for line in contents {
        priority_sum += get_common_item_priority(line);
    }

    println!("{priority_sum}");
}

