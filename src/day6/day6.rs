use std::collections::HashSet;
use std::fs;

fn find_unique_window(chars: Vec<char>, window_size: usize) -> u16 {
    chars
        .windows(window_size)
        .zip(0..)
        .filter(|(window, i)| window.iter().collect::<HashSet<_>>().len() == window_size)
        .next()
        .map(|(_, i)| (i + window_size) as u16)
        .unwrap()
}

pub fn run() {
    // let file_path = "src/day6/example_input.txt";
    let file_path = "src/day6/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let chars = contents.chars().collect::<Vec<_>>();

    let part_1 = find_unique_window(chars.clone(), 4);
    println!("part 1 result: {:?}", part_1);

    let part_2 = find_unique_window(chars, 14);
    println!("part 2 result: {:?}", part_2);
}
