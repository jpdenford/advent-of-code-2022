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

fn exec_moves_part_1(stacks: Vec<Vec<char>>, moves: Vec<(u8, u8, u8)>) -> Vec<Vec<char>> {
    let mut new_stacks = stacks.clone();

    for (num_to_move, from, to) in moves {
        for _ in 0..num_to_move {
            let popped = new_stacks[(from - 1) as usize].pop().unwrap();
            new_stacks[(to - 1) as usize].push(popped);
        }
    }
    new_stacks
}

fn exec_moves_part_2(stacks: Vec<Vec<char>>, moves: Vec<(u8, u8, u8)>) -> Vec<Vec<char>> {
    let mut new_stacks = stacks.clone();

    for (num_to_move, from, to) in moves {
        let from_stack = &mut new_stacks[(from - 1) as usize];
        let target_length = &from_stack.len().saturating_sub((num_to_move) as usize);
        let mut popped = from_stack.split_off(target_length.clone());
        new_stacks[(to - 1) as usize].append(&mut popped);
    }
    new_stacks
}

fn get_top_of_stacks(stacks: Vec<Vec<char>>) -> String {
    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
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

    let final_stacks_1 = exec_moves_part_1(stacks.clone(), moves.clone());
    let part_1 = get_top_of_stacks(final_stacks_1);
    println!("part 1 result: {:?}", part_1);

    let final_stacks_2 = exec_moves_part_2(stacks, moves);
    let part_2 = get_top_of_stacks(final_stacks_2);
    println!("part 2 result: {:?}", part_2);
}
