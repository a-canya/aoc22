use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE_PATH).expect("Could not read input file");

    let mut max_calories = 0;
    let mut current_elf_calories = 0;
    for line in input.split("\n") {
        if line == "" {
            max_calories = max(max_calories, current_elf_calories);
            current_elf_calories = 0;
        } else {
            let current_snack_calories = line
                .parse::<i32>()
                .expect("Could not parse integer in one line");
            current_elf_calories += current_snack_calories;
        }
    }
    max_calories = max(max_calories, current_elf_calories);

    println!("{max_calories}");
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}
