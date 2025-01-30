use regex::Regex;
use std::error::Error;
use std::str::FromStr;

use advent_of_code_2024::file_reader::get_input;
// use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(i32, i32),
}

impl Instruction {
    fn execute(&self) -> i32 {
        match self {
            Instruction::Mul(a, b) => a * b,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TobogganComputer {
    instructions: Vec<Instruction>,
}

impl FromStr for TobogganComputer {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instructions = vec![];

        let mul_regex = Regex::new(r"mul\((?<number1>\d{1,3}),(?<number2>\d{1,3})\)").unwrap();

        // this matches the inactive parts of the code
        // I had to add the non greedy part (.*?) to make
        // sure that not all of the input is split away

        let dont_regex = Regex::new(r"don't\(\)(.*?)do\(\)").unwrap();

        let active_strings: Vec<&str> = dont_regex.split(s).collect();

        for string in active_strings {
            for capture in mul_regex.captures_iter(string) {
                instructions.push(Instruction::Mul(
                    capture["number1"].parse().unwrap(),
                    capture["number2"].parse().unwrap(),
                ));
            }
        }
        Ok(TobogganComputer { instructions })
    }
}

impl TobogganComputer {
    fn execute(&self) -> i32 {
        self.instructions
            .iter()
            .map(|instruction| instruction.execute())
            .sum()
    }
}

fn main() {
    let input = get_input("src/inputs/input_day03.txt");
    let computer = TobogganComputer::from_str(&input).unwrap();
    // println!(
    //     "The found multiplication instructions are: {:?}",
    //     computer.instructions
    // );

    dbg!(&computer.instructions);
    let result = computer.execute();
    println!(
        "The result coming from the computer for part a is: {}",
        result
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_conditionals() {
        let test_str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let test_computer = TobogganComputer::from_str(test_str).unwrap();
        let result_instructions = vec![Instruction::Mul(2, 4), Instruction::Mul(8, 5)];
        assert_eq!(test_computer.instructions, result_instructions);
    }

    #[test]
    fn test_mul_from_string() {
        let test_str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let test_computer = TobogganComputer::from_str(test_str).unwrap();
        let result = 161;
        assert_eq!(test_computer.execute(), result);
    }

    #[test]
    fn test_from_string() {
        let test_str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let test_computer = TobogganComputer::from_str(test_str).unwrap();
        let result_instructions = vec![
            Instruction::Mul(2, 4),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Mul(8, 5),
        ];
        assert_eq!(test_computer.instructions, result_instructions);
    }

    #[test]
    fn simple_test_mul() {
        let test_instructions = vec![Instruction::Mul(3, 8), Instruction::Mul(2, 2)];
        let result = 28;
        let test_computer = TobogganComputer {
            instructions: test_instructions,
        };
        assert_eq!(result, test_computer.execute());
    }

    #[test]
    fn simple_test_mul_add() {
        let test_instructions = vec![Instruction::Mul(7, 2), Instruction::Mul(2, 2)];
        let result = 51;
        let test_computer = TobogganComputer {
            instructions: test_instructions,
        };
        assert_eq!(result, test_computer.execute());
    }
}
