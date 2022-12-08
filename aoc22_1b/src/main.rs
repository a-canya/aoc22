use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE_PATH).expect("Could not read input file");

    let mut top_3_elves_calories = [0, 0, 0];

    let mut current_elf_calories = 0;

    for line in input.split("\n") {
        if line == "" {
            update_top_3(&mut top_3_elves_calories, current_elf_calories);
            current_elf_calories = 0;
        } else {
            let current_snack_calories = line
                .parse::<i32>()
                .expect("Could not parse integer in one line");
            current_elf_calories += current_snack_calories;
        }
    }
    update_top_3(&mut top_3_elves_calories, current_elf_calories);

    println!("top 3: {:?}", top_3_elves_calories);

    let top_3_elves_calories_added: i32 = top_3_elves_calories.iter().sum();
    println!("summed: {top_3_elves_calories_added}");
}

fn update_top_3(top_3: &mut [i32; 3], current: i32) {
    if current <= top_3[2] {
        return;
    }
    if current <= top_3[1] {
        top_3[2] = current;
        return;
    }
    if current <= top_3[0] {
        top_3[2] = top_3[1];
        top_3[1] = current;
        return;
    }
    top_3[2] = top_3[1];
    top_3[1] = top_3[0];
    top_3[0] = current;
}
