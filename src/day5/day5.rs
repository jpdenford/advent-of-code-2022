use regex::Regex;
use std::fs;

fn parse_stacks(stack_def: &str) -> Vec<Vec<char>> {
    let lines = stack_def.split('\n').collect::<Vec<&str>>();
    let (setups, columns) = lines.split_at(lines.len() - 1);
    let sets = columns
        .get(0)
        .unwrap()
        .chars()
        .filter(|ch| ch != &' ')
        .count();

    let setups: Vec<&str> = setups.into_iter().cloned().rev().collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..sets {
        stacks.push(Vec::new());
    }

    println!("stacks len {:?}", stacks.len());
    // we want to push things on
    // in the order seen in the file
    // setups.reverse();
    for row in setups {
        let chars = row.chars().collect::<Vec<char>>();
        for i in 0..stacks.len() {
            let ch = chars.get(i * 4 + 1);
            println!("got {:?} {:?} {:?}", row, i, ch);
            match ch {
                Some(chr) if chr != &' ' => stacks[i].push(chr.clone()),
                _ => {}
            }
        }
    }
    println!("init stacks: {:?}", stacks);
    stacks
}

// returns a list of moves (n, from,  to)
fn parse_moves(moves_def: &str) -> Vec<(u8, u8, u8)> {
    let num_regx = Regex::new(r"(\d+)").unwrap();
    moves_def
        .split('\n')
        .map(|line| {
            num_regx
                .captures_iter(line)
                .map(|cap| cap[0].parse::<u8>().unwrap())
            // .line
            // .chars()
            // .filter(|ch| ch.is_ascii_digit())
            // .map(|c| c.to_digit(10).unwrap())
        })
        .map(|mut nums| {
            (
                nums.nth(0).unwrap() as u8,
                nums.nth(0).unwrap() as u8,
                nums.nth(0).unwrap() as u8,
            )
        })
        .collect()
}

fn exec_moves(stacks: Vec<Vec<char>>, moves: Vec<(u8, u8, u8)>) -> Vec<Vec<char>> {
    let mut new_stacks = stacks.clone();

    for (num_to_move, from, to) in moves {
        for _ in 0..num_to_move {
            let popped = new_stacks[(from - 1) as usize].pop().unwrap();
            new_stacks[(to - 1) as usize].push(popped);
        }
    }
    new_stacks
}

pub fn run() {
    // let file_path = "src/day5/example_input.txt";
    let file_path = "src/day5/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let parts = contents.split("\n\n").collect::<Vec<_>>();

    let stack_def = parts.get(0).unwrap();
    let moves_def = parts.get(1).unwrap();

    let stacks = parse_stacks(stack_def);
    let moves = parse_moves(moves_def);

    let final_stacks = exec_moves(stacks, moves);
    let result = final_stacks
        .iter()
        .map(|s| s.last().unwrap())
        .collect::<String>();
    println!("part 1 result: {:?}", result);
    // println!("part 2 result: {:?}", part_2);
}
