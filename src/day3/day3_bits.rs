use std::collections::HashSet;
use std::fs;

pub fn run() {
    let file_path = "src/day3/example_input.txt";
    // let file_path = "src/day3/input.txt";
    let contents = fs::read_to_string(file_path).expect("Couldn't open file!");

    let x: Vec<u64> = contents
        .split("\n")
        .map(|line| {
            let bit_fields: Vec<u8> = line
                .chars()
                .map(|chr| {
                    if chr.is_lowercase() {
                        chr as u8 - 96
                    } else {
                        chr as u8 - 38
                    }
                })
                // .map(|n| 1u64 << n)
                .collect();
            let (first_bucket, second_bucket) = bit_fields.split_at(bit_fields.len() / 2);
            let bit_first = first_bucket
                .iter()
                .fold(0u64, |bit_field, num| bit_field | (1 << num));
            let bit_second = second_bucket
                .iter()
                .fold(0u64, |bit_field, num| bit_field | (1 << num));
            println!("{:?}", bit_first);
            let shared = bit_first & bit_second;

            shared
        })
        .collect();
    x.map(|num_set| {
        let value = 100u32;
        let mask = 1u32; // assuming rightmost bit first
        for i in 0..32 {
            let is_set = value & mask != 0;
            mask <<= 1; // assuming rightmost bit first
        }
    });

    println!("part 1 result: {:?}", x);
    // println!("{:?}", 'a' as u32 - 96);
    // println!("{:?}", 'Z' as u32 - 38)
}

// vJrwpWtwJgWr
// hcsFMMfFFhFp
