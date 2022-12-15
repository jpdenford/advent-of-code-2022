use std::fs;

fn parse_range(str: &str) -> (u8, u8) {
    let mut x = str.split('-').map(|bound| bound.parse::<u8>().unwrap());
    // first & second elements
    return (x.nth(0).unwrap(), x.nth(0).unwrap());
}

fn fully_contained((x, y): ((u8, u8), (u8, u8))) -> bool {
    (x.0 <= y.0 && y.1 <= x.1) || (y.0 <= x.0 && x.1 <= y.1)
}

fn overlapping((x, y): ((u8, u8), (u8, u8))) -> bool {
    u8::max(x.0, y.0) <= u8::min(x.1, y.1)
}

pub fn run() {
    // let file_path = "src/day4/example_input.txt";
    let file_path = "src/day4/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let part_1: usize = contents
        .trim()
        .split("\n")
        .map(|line| line.split(",").map(parse_range).clone().collect::<Vec<_>>())
        .map(|ab| (ab.get(0).cloned().unwrap(), ab.get(1).cloned().unwrap()))
        .map(fully_contained)
        .filter(|c| c.clone())
        .count();

    println!("part 1 result: {:?}", part_1);

    let part_2: usize = contents
        .trim()
        .split("\n")
        .map(|line| line.split(",").map(parse_range).clone().collect::<Vec<_>>())
        .map(|ab| (ab.get(0).cloned().unwrap(), ab.get(1).cloned().unwrap()))
        .map(overlapping)
        .filter(|c| c.clone())
        .count();

    println!("part 2 result: {:?}", part_2);
}
