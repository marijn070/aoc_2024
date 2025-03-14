use advent_of_code_2024::file_reader;
use nom::{
    self,
    branch::alt,
    bytes::{complete::tag, take_while},
    character::{
        complete::{i32, line_ending, space1},
        one_of,
    },
    sequence::{preceded, separated_pair, terminated},
    AsChar, IResult, Parser,
};

const A_BUTTON_COST: i32 = 3;
const B_BUTTON_COST: i32 = 1;
const MAX_BUTTON_PRESSES: u8 = 100;

fn main() {
    let input = file_reader::get_input("src/inputs/test_day13.txt");
    println!("{input}");

    // i will be putting the values into an array
    // [xa, ya, xb, yb, x_prize, y_prize]

    let xa = 17;
    let ya = 86;
    let xb = 84;
    let yb = 37;

    let x_prize = 7870;
    let y_prize = 6450;

    let presses_a = -(xb * y_prize - yb * x_prize) / (xa * yb - ya * xb);
    let presses_b = -(ya * x_prize - xa * y_prize) / (xa * yb - ya * xb);

    println!("we need to press button A {presses_a} times");
    println!("we need to press button B {presses_b} times");

    println!("{grab_machine("Button A: X+30, Y+13"):?}");
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

fn grab_machine(input: &str) -> IResult<&str, GrabMachine> {
    // first implement the basic parsers
    let x_val = preceded(
        alt((tag("X+"), tag("X="))),
        take_while(AsChar::is_dec_digit),
    )
    .map_res(|s: &str| s.parse::<i32>());

    let y_val = preceded(
        alt((tag("Y+"), tag("Y="))),
        take_while(AsChar::is_dec_digit),
    )
    .map_res(|s: &str| s.parse::<i32>());

    let button_a = tag("Button A: ");
    let button_b = tag("Button B: ");
    let prize = tag("Prize: ");

    let separator = (tag(","), space1);

    let (input, (_, x_a, _, y_a)) = (button_a, x_val, &separator, y_val).parse(input)?;
    let (input, (_, x_b, _, y_b)) = (button_b, x_val, separator, y_val).parse(input)?;
    let (input, (_, x_prize, _, y_prize)) = (prize, x_val, separator, y_val).parse(input)?;

    Ok((
        input,
        GrabMachine {
            a: (x_a, y_a),
            b: (x_b, y_b),
            prize: (x_prize, y_prize),
        },
    ))
}
