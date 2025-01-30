use advent_of_code_2024::file_reader;
use std::collections::HashMap;

fn main() {
    let input = file_reader::get_input("src/inputs/input_day11.txt");

    let mut stones: HashMap<u64, u64> = HashMap::new();

    input
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .for_each(|x| {
            stones.entry(x).and_modify(|v| *v += 1).or_insert(1);
        });

    let n_blinks = 75;

    for _ in 0..n_blinks {
        let mut new_stones: HashMap<u64, u64> = HashMap::new();
        for (stone, count) in stones.iter() {
            let stone_result = blink(*stone);
            for result in stone_result {
                new_stones
                    .entry(result)
                    .and_modify(|v| *v += *count)
                    .or_insert(*count);
            }
        }
        stones = new_stones;
    }

    let answer_a: u64 = stones.values().sum();

    println!("after blinking {n_blinks} times, there are {answer_a} stones");
}

fn blink(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    };
    let stone_str = stone.to_string();
    let n_digits = stone_str.len();
    if n_digits % 2 == 0 {
        let mid = n_digits / 2;
        let (left, right) = stone_str.split_at(mid);
        return vec![left.parse::<u64>().unwrap(), right.parse::<u64>().unwrap()];
    }
    vec![stone * 2024]
}
