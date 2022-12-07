use std::fs;

fn parse_game_line(line: &str) -> ((char, u32), (char, u32)) {
    let chars: Vec<char> = line.clone().chars().collect();

    let their_input = chars.get(0).unwrap();
    let their_move: u32 = match their_input {
        'A' => 1, // rock
        'B' => 2, // paper
        'C' => 3, // scissors
        _ => panic!("unsupported char"),
    };

    let our_input = chars.get(2).unwrap();
    let our_move: u32 = match our_input {
        'X' => 1, // rock
        'Y' => 2, // paper
        'Z' => 3, // scissors
        _ => panic!("unsupported char"),
    };

    // tuple with original characters & corresponding move
    (
        (their_input.clone(), their_move),
        (our_input.clone(), our_move),
    )
}

fn calc_score(a: u32, b: u32) -> u32 {
    match (a, b) {
        (a, b) if a == b => 3 + b, // same = draw
        (2, 1) => b,               // Paper vs Rock -> they win
        (3, 2) => b,               // Scissors vs Paper -> they win
        (1, 3) => b,               // Rock vs Scissors -> they win
        _ => 6 + b,                // every other combo -> b wins
    }
}

fn calc_score_from_desired(their_move: u32, desired_outcome: char) -> u32 {
    let desired_move = match desired_outcome {
        // a losing move is one 'higher' (wrapped around e.g. Rock 1 -> Paper 2)
        'X' => ((their_move + 1) % 3) + 1,
        'Y' => their_move, // a draw is the same move
        // a winning move is one 'lower' (wrapped around e.g. Rock 1 -> Scissors 3)
        'Z' => (their_move % 3) + 1,
        _ => panic!(),
    };
    calc_score(their_move, desired_move)
}

// Keeping this very concrete because I know part two will change things
pub fn run() {
    // let file_path = "src/day2/example_input.txt";
    let file_path = "src/day2/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let games = contents.trim().split("\n").map(parse_game_line);

    let part_1: u32 = games.clone().map(|(a, b)| calc_score(a.1, b.1)).sum();

    let part_2: u32 = games
        .clone()
        .map(|(their_move, our_move)| calc_score_from_desired(their_move.1, our_move.0))
        .sum();

    println!("part 1 result: {:?}", part_1);
    println!("part 2 result: {:?}", part_2);
}
