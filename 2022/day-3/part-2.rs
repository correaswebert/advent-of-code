use std::env;
use std::fs;

fn get_priority(item: char) -> u32 {
    match item {
        'a'..='z' => return item as u32 - 'a' as u32,
        'A'..='Z' => return item as u32 - 'A' as u32 + 26,
        _ => return 0,
    }
}

fn get_common_item_priority(elf_1: &String, elf_2: &String, elf_3: &String) -> u64 {
    let mut items_1: u64 = 0;
    let mut items_2: u64 = 0;
    let mut items_3: u64 = 0;

    for item in elf_1.chars() {
        items_1 |= 1 << get_priority(item);
    }

    for item in elf_2.chars() {
        items_2 |= 1 << get_priority(item);
    }

    for item in elf_3.chars() {
        items_3 |= 1 << get_priority(item);
    }

    let mut common_item = items_1 & items_2 & items_3;
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

    for i in 0..contents.len() / 3 {
        priority_sum +=
            get_common_item_priority(&contents[3 * i], &contents[3 * i + 1], &contents[3 * i + 2]);
    }

    println!("{priority_sum}");
}
