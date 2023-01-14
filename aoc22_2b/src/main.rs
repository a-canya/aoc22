use std::fs;

const INPUT_FILE_PATH: &str = "input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE_PATH).expect("could not read input file");

    let mut score = 0;

    for line in input.split("\n") {
        // rock=0, paper=1, scissors=2
        let opponent = (line.bytes().nth(0).unwrap() - b'A') as i32;

        let winner_score = (line.bytes().nth(2).unwrap() - b'X') as i32 * 3;

        let us = match winner_score {
            0 => modulo(opponent - 1, 3),
            3 => opponent,
            6 => (opponent + 1) % 3,
            x => panic!("we got a winner_score not in {{0,3,6}}: {}", x),
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
