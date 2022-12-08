use std::collections::HashSet;
use std::fs;

fn as_priority(chr: char) -> u32 {
    if chr.is_lowercase() {
        chr as u32 - 96
    } else {
        chr as u32 - 38
    }
}

fn intersection(acc: HashSet<u32>, cur: HashSet<u32>) -> HashSet<u32> {
    acc.intersection(&cur).cloned().collect::<HashSet<u32>>()
}

pub fn run() {
    // let file_path = "src/day3/example_input.txt";
    let file_path = "src/day3/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let part_1: u32 = contents
        .split("\n")
        .map(|line| {
            let priorities: Vec<u32> = line.chars().map(as_priority).collect();
            let (ruck_1, ruck_2) = priorities.split_at(priorities.len() / 2);

            let ruckset_1: HashSet<u32> = ruck_1.iter().cloned().collect();
            let ruckset_2: HashSet<u32> = ruck_2.iter().cloned().collect();
            (ruckset_1, ruckset_2)
        })
        .map(|(x, y)| x.intersection(&y).cloned().collect::<Vec<u32>>())
        .flatten()
        .sum();

    println!("part 1 result: {:?}", part_1);

    let part_2: u32 = contents
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|three_lines| {
            three_lines
                .iter()
                .map(|line| line.chars().map(as_priority).collect::<HashSet<u32>>())
        })
        .map(|three_sets| three_sets.reduce(intersection))
        .flat_map(|shared| shared.unwrap().into_iter())
        .sum();

    println!("part 2 result: {:?}", part_2);
}
