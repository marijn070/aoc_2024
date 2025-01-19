use std::collections::HashSet;

use advent_of_code_2024::file_reader;
use strum::VariantMetadata;

fn main() {
    let input = file_reader::get_input("src/inputs/input_day07.txt");

    let mut bridge_equations: Vec<BridgeEquation> = vec![];

    // parse the input into a vector of bridge equations
    for line in input.lines() {
        if let Some((test_value, equation_str)) = line.split_once(": ") {
            let test_value = test_value.parse::<u32>().unwrap();
            let equation: Vec<u32> = equation_str
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            bridge_equations.push(BridgeEquation {
                test_value,
                equation,
            });
        }
    }

    let total_calibration_result: u32 = bridge_equations
        .iter()
        .filter(|bridge_equation| {
            let possibilities = get_equation_possibilities(&bridge_equation.equation);
            possibilities.contains(&bridge_equation.test_value)
        })
        .map(|bridge_equation| bridge_equation.test_value)
        .sum();

    println!("The total sum of the correct equations is {total_calibration_result}");
}

fn get_equation_possibilities(equation: &[u32]) -> HashSet<u32> {
    let mut possibilities = HashSet::new();

    if equation.len() == 1 {
        possibilities.insert(equation[0]);
        return possibilities;
    }

    let last_value = equation.last().copied().unwrap();
    let remaining_equation = &equation[..equation.len() - 1];
    let previous_possibilities = get_equation_possibilities(remaining_equation);

    for possibility in previous_possibilities {
        possibilities.insert(last_value * possibility);
        possibilities.insert(last_value + possibility);
    }

    possibilities
}

#[derive(Debug)]
struct BridgeEquation {
    test_value: u32,
    equation: Vec<u32>,
}
