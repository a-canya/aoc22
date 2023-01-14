use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE_PATH).expect("could not read input file");

    let mut score = 0;

    for line in input.split("\n") {
        // rock=0, paper=1, scissors=2
        let opponent = (line.bytes().nth(0).unwrap() - b'A') as i32;
        let us = (line.bytes().nth(2).unwrap() - b'X') as i32;

        let winner_score = match modulo(us - opponent, 3) {
            0 => 3,
            1 => 6,
            2 => 0,
            x => panic!("module 3 operation must give 0, 1 or 2, but gave {}", x),
        };

        let shape_score = us + 1;

        let round_score = winner_score + shape_score;

        score += round_score;

        // print!(
        //     "line: {}. shape {}, winner {}, round {}, acc {}\n",
        //     line, shape_score, winner_score, round_score, score
        // );
    }

    print!("{}\n\n", score)
}

fn modulo(x: i32, y: i32) -> i32 {
    ((x % y) + y) % y
}
