use std::collections::HashSet;

use advent_of_code_2024::file_reader;

fn main() {
    let input = file_reader::get_input("src/inputs/input_day07.txt");

    let mut bridge_equations: Vec<BridgeEquation> = vec![];

    // parse the input into a vector of bridge equations
    for line in input.lines() {
        if let Some((test_value, equation_str)) = line.split_once(": ") {
            let test_value = test_value.parse::<u64>().unwrap();
            let equation: Vec<u64> = equation_str
                .split_whitespace()
                .filter_map(|s| s.parse::<u64>().ok())
                .collect();
            bridge_equations.push(BridgeEquation {
                test_value,
                equation,
            });
        }
    }

    let answer_a = get_total_calibration_result(&bridge_equations, false);
    let answer_b = get_total_calibration_result(&bridge_equations, true);

    println!("The total sum of the correct equations is {answer_a}");
    println!("The total sum of the correct equations including the concat operator is {answer_b}");
}

fn get_total_calibration_result(bridge_equations: &[BridgeEquation], concat_included: bool) -> u64 {
    bridge_equations
        .iter()
        .filter(|bridge_equation| {
            let possibilities =
                get_equation_possibilities(&bridge_equation.equation, concat_included);
            possibilities.contains(&bridge_equation.test_value)
        })
        .map(|bridge_equation| bridge_equation.test_value)
        .sum()
}

fn get_equation_possibilities(equation: &[u64], concat_included: bool) -> HashSet<u64> {
    let mut possibilities = HashSet::new();

    if equation.len() == 1 {
        possibilities.insert(equation[0]);
        return possibilities;
    }

    let last_value = equation.last().copied().unwrap();
    let remaining_equation = &equation[..equation.len() - 1];
    let previous_possibilities = get_equation_possibilities(remaining_equation, concat_included);

    for possibility in previous_possibilities {
        possibilities.insert(possibility * last_value);
        possibilities.insert(possibility + last_value);
        if concat_included {
            possibilities.insert(concat_operator(possibility, last_value));
        };
    }

    possibilities
}

fn concat_operator(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.to_string().len() as u32) + b
}

#[derive(Debug)]
struct BridgeEquation {
    test_value: u64,
    equation: Vec<u64>,
}
