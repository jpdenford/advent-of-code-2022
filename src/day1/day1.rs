use std::fs;

pub fn run() {
    println!("Running - day 1");

    // let file_path = "src/1/example_input.txt";
    let file_path = "src/day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let mut elf_calories_dec: Vec<i32> = contents
        .split("\n\n")
        .map(|elf_block| {
            elf_block
                .split("\n")
                .map(|num_str| num_str.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    elf_calories_dec.sort_by(|n1, n2| n2.partial_cmp(n1).unwrap());

    let max_calories = elf_calories_dec.first().unwrap();
    println!("day1 part1 answer: {}", max_calories);

    let sum_top_three: i32 = elf_calories_dec.iter().take(3).sum();
    println!("day1 part2 answer: {}", sum_top_three);
}
