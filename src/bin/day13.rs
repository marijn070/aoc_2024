use advent_of_code_2024::file_reader;
use regex::Regex;

const A_BUTTON_COST: i64 = 3;
const B_BUTTON_COST: i64 = 1;
const MAX_BUTTON_PRESSES: u8 = 100;

fn main() {
    let input = file_reader::get_input("src/inputs/input_day13.txt");

    let machines: Vec<GrabMachine> = input
        .split("\n\n")
        .map(|s| {
            let re = Regex::new(r"\d+").unwrap();
            let coords: Vec<i64> = re
                .captures_iter(s)
                .map(|c| c[0].parse::<i64>().unwrap())
                .take(6)
                .collect();
            GrabMachine {
                a: (coords[0], coords[1]),
                b: (coords[2], coords[3]),
                prize: (coords[4], coords[5]),
            }
        })
        .collect();

    let machine_tokens: Vec<i64> = machines
        .iter()
        .map(|m| m.calculate_grab_tokens(true))
        .collect();
    let answer_a: i64 = machine_tokens.iter().sum();

    println!("The answer to part a is {answer_a:?}");

    let updated_machines: Vec<GrabMachine> = machines
        .iter()
        .map(|m| GrabMachine {
            prize: (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000),
            ..m.clone()
        })
        .collect();

    let updated_machine_tokens: Vec<i64> = updated_machines
        .iter()
        .map(|m| m.calculate_grab_tokens(false))
        .collect();

    let answer_b: i64 = updated_machine_tokens.iter().sum();

    println!("The answer to part b is {answer_b:?}");
}

#[derive(Debug, Clone)]
struct GrabMachine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl GrabMachine {
    fn calculate_grab_tokens(&self, press_limit: bool) -> i64 {
        let a0 = self.a.0;
        let a1 = self.a.1;
        let b0 = self.b.0;
        let b1 = self.b.1;

        // Prize coordinates
        let prize0 = self.prize.0;
        let prize1 = self.prize.1;

        // Determinant of the coefficient matrix
        let det = a0 * b1 - a1 * b0;

        // Calculate the numerator for presses_a and presses_b
        let num_presses_a = -(b0 * prize1 - b1 * prize0);
        let num_presses_b = -(a1 * prize0 - a0 * prize1);

        // Check if the results are integers
        if num_presses_a % det != 0 || num_presses_b % det != 0 {
            return 0; // No valid integer solution
        }

        // Calculate the number of presses for each button
        let presses_a = num_presses_a / det;
        let presses_b = num_presses_b / det;

        if !press_limit || (presses_a <= 100 && presses_b <= 100) {
            A_BUTTON_COST * presses_a + B_BUTTON_COST * presses_b
        } else {
            0
        }
    }
}
