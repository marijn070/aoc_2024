use advent_of_code_2024::file_reader;
use regex::Regex;

const A_BUTTON_COST: i32 = 3;
const B_BUTTON_COST: i32 = 1;
const MAX_BUTTON_PRESSES: u8 = 100;

fn main() {
    let input = file_reader::get_input("src/inputs/test_day13.txt");
    println!("{input}");

    let machines: Vec<GrabMachine> = input
        .split("\n\n")
        .map(|s| {
            let re = Regex::new(r"\d+").unwrap();
            let coords: Vec<i32> = re
                .captures_iter(s)
                .map(|c| c[0].parse::<i32>().unwrap())
                .take(6)
                .collect();
            GrabMachine {
                a: (coords[0], coords[1]),
                b: (coords[2], coords[3]),
                prize: (coords[4], coords[5]),
            }
        })
        .collect();

    println!("{machines:?}");
}

#[derive(Debug)]
struct GrabMachine {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

impl GrabMachine {
    fn calculate_grab_tokens(&self) -> i32 {
        let presses_a = -(self.b.0 * self.prize.1 - self.b.1 * self.prize.0)
            / (self.a.0 * self.b.1 - self.a.1 * self.b.0);
        let presses_b = -(self.a.1 * self.prize.0 - self.a.0 * self.prize.1)
            / (self.a.0 * self.b.1 - self.a.1 * self.b.0);

        if presses_a <= 100 || presses_b <= 100 {
            A_BUTTON_COST * presses_a + B_BUTTON_COST * presses_b
        } else {
            0
        }
    }
}
