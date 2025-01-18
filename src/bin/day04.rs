use std::error::Error;
use std::fmt;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use advent_of_code_2024::file_reader;

#[derive(Debug)]
struct WordSearch {
    pub word: String,
    grid: Vec<Vec<char>>,
    period_grid: Vec<Vec<char>>,
}

impl FromStr for WordSearch {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        for line in s.lines() {
            let mut line_vec: Vec<char> = vec![];
            for char in line.chars() {
                line_vec.push(char);
            }
            grid.push(line_vec);
        }
        Ok(WordSearch {
            word: String::from(""),
            grid: grid.clone(),
            period_grid: vec![vec!['.'; grid[0].len()]; grid.len()],
        })
    }
}
impl fmt::Display for WordSearch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grid:")?;
        for row in &self.grid {
            for &char in row {
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Period Grid:")?;
        for row in &self.period_grid {
            for &char in row {
                write!(f, "{}", char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, EnumIter)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl WordSearch {
    fn problem_a(&mut self) -> u32 {
        let mut n_occurrences_word = 0;
        let mut first_letter_coords: Vec<(usize, usize)> = vec![];
        for (i, row) in self.grid.iter().enumerate() {
            for (j, character) in row.iter().enumerate() {
                if *character == self.word.chars().nth(0).unwrap() {
                    first_letter_coords.push((i, j));
                }
            }
        }

        for coord in first_letter_coords {
            for direction in Direction::iter() {
                if self.search_word_in_direction((coord.0, coord.1), direction) {
                    n_occurrences_word += 1;
                }
            }
        }
        return n_occurrences_word;
    }

    fn problem_b(&mut self) -> Result<u32, &str> {
        if self.word.len() != 3 {
            return Err("the problem is looking for the word {}, which does not have 3 characters");
        }
        let mut n_occurrences_word = 0;

        let middle_letter = self.word.chars().nth(1).unwrap();
        let mut middle_letter_coords: Vec<(usize, usize)> = vec![];

        for (i, row) in self.grid.iter().enumerate() {
            for (j, character) in row.iter().enumerate() {
                if *character == middle_letter {
                    middle_letter_coords.push((i, j));
                }
            }
        }

        for coord in middle_letter_coords {
            if self.search_word_x(coord) {
                n_occurrences_word += 1;
            }
        }

        Ok(n_occurrences_word)
    }

    fn search_word_x(&mut self, coords: (usize, usize)) -> bool {
        // we are looking for the middle of the word (assuming the word has lenghth 3),
        // and then checking the four corners

        let mut chars = self.word.chars();
        let first_char = chars.next().unwrap();
        let second_char = chars.next().unwrap();
        let third_char = chars.next().unwrap();

        if self.grid[coords.0][coords.1] != second_char {
            return false;
        }

        if coords.0 <= 0
            || coords.0 >= self.grid.len() - 1
            || coords.1 <= 0
            || coords.1 >= self.grid[0].len() - 1
        {
            return false;
        }

        let corner_coordinates = vec![
            (coords.0 - 1, coords.1 - 1),
            (coords.0 - 1, coords.1 + 1),
            (coords.0 + 1, coords.1 + 1),
            (coords.0 + 1, coords.1 - 1),
        ];

        let corner_characters: Vec<char> = corner_coordinates
            .iter()
            .map(|(i, j)| self.grid[*i][*j])
            .collect();

        // Count occurrences of the two characters using an iterator
        let (count1, count2) = corner_characters.iter().fold((0, 0), |(c1, c2), &ch| {
            if ch == first_char {
                (c1 + 1, c2)
            } else if ch == third_char {
                (c1, c2 + 1)
            } else {
                (c1, c2) // Ignore other characters
            }
        });

        // Check that both characters appear exactly twice
        if !(count1 == 2 && count2 == 2) {
            return false;
        }

        let has_adjacent_duplicates = corner_characters.windows(2).any(|pair| pair[0] == pair[1]);
        if !has_adjacent_duplicates {
            return false;
        }

        // if we reach here we can fill in the period grid
        self.period_grid[coords.0][coords.1] = second_char;
        for (coords, char) in corner_coordinates.iter().zip(corner_characters.iter()) {
            self.period_grid[coords.0][coords.1] = *char;
        }

        true
    }

    fn search_word_in_direction(&mut self, coords: (usize, usize), direction: Direction) -> bool {
        // this function will look if we match the word going right
        let mut cursor = (coords.0 as isize, coords.1 as isize);
        let mut positions = vec![];

        for (_i, word_char) in self.word.chars().enumerate() {
            // see if the word character is matched
            if cursor.0 < 0
                || cursor.1 < 0
                || cursor.0 >= self.grid.len() as isize
                || cursor.1 >= self.grid[0].len() as isize
            {
                return false;
            }

            if self.grid[cursor.0 as usize][cursor.1 as usize] != word_char {
                return false;
            }

            // store the position
            positions.push((cursor.0 as usize, cursor.1 as usize));

            // update the cursor
            let (dx, dy) = match direction {
                Direction::N => (-1, 0),
                Direction::NE => (-1, 1),
                Direction::E => (0, 1),
                Direction::SE => (1, 1),
                Direction::S => (1, 0),
                Direction::SW => (1, -1),
                Direction::W => (0, -1),
                Direction::NW => (-1, -1),
            };

            cursor.0 += dx;
            cursor.1 += dy;
        }

        for (i, pos) in positions.iter().enumerate() {
            self.period_grid[pos.0][pos.1] = self.word.chars().nth(i).unwrap();
        }
        true
    }
}

fn main() {
    let input = file_reader::get_input("src/inputs/input_day04.txt");
    let mut wordsearch_a = WordSearch::from_str(&input).unwrap();
    let mut wordsearch_b = WordSearch::from_str(&input).unwrap();

    wordsearch_a.word = String::from("XMAS");
    wordsearch_b.word = String::from("MAS");

    let sol_problem_a = wordsearch_a.problem_a();
    println!("{}", wordsearch_a);
    println!("The solution for question a is : {sol_problem_a}");

    let sol_prolem_b = wordsearch_b.problem_b();
    println!("The solution for question b is : {}", sol_prolem_b.unwrap());
    // println!("{}", wordsearch_b);
}
