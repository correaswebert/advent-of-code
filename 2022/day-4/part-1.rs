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

    let mut contained_sections = 0;

    for line in contents {
        let mut sections = line.split(",");

        let mut elf_1 = sections.next().unwrap().split("-");
        let elf_1_lower: u32 = elf_1.next().unwrap().parse().unwrap();
        let elf_1_higher: u32 = elf_1.next().unwrap().parse().unwrap();

        let mut elf_2 = sections.next().unwrap().split("-");
        let elf_2_lower: u32 = elf_2.next().unwrap().parse().unwrap();
        let elf_2_higher: u32 = elf_2.next().unwrap().parse().unwrap();

        if (elf_1_lower <= elf_2_lower && elf_1_higher >= elf_2_higher)
            || (elf_2_lower <= elf_1_lower && elf_2_higher >= elf_1_higher)
        {
            contained_sections += 1;
        }
    }

    println!("{contained_sections}");
}
