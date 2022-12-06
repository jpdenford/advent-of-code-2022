use std::fs;

pub fn run() {
    println!("Running - day 1");

    // let file_path = "src/1/example_input.txt";
    let file_path = "src/day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let max_calorie: i32 = contents
        .split("\n\n")
        .map(|reindeer_block| {
            reindeer_block
                .split("\n")
                .map(|numstr| numstr.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap();

    println!("day1 answer: {}", max_calorie);
}
