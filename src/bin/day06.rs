use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use advent_of_code_2024::file_reader;
use grid::*;

fn main() {
    let input = file_reader::get_input("src/inputs/input_day06.txt");

    let mut suitmaplab = SuitLabMap::from_str(&input).unwrap();

    loop {
        let finished = suitmaplab.step();
        if finished {
            break;
        }
    }

    let answer_a = suitmaplab.get_n_guard_positions();
    println!("the guard has visited {answer_a} unique positions");
    dbg!(suitmaplab.get_annotated_map());

    // for part b we need to obstruct the guard somewhere to make
    // him walk in a loop.
    //   1. where do we check?
    //   2. how do we know a loop has been made?
    //
    // for the first quesiton, I think that we should only check all or the
    // locations in the guards unobstructed path history, as those will ensure
    // a collision.
    // for the second question, we can see if the guard revisits a point facing the same direction again, meaning that it has reached a loop.
    // Therefore, we must also record the direction the guard had in each point.
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn step_direction(&self) -> (isize, isize) {
        use Orientation::*;
        match self {
            Up => (-1, 0),
            Right => (0, 1),
            Down => (1, 0),
            Left => (0, -1),
        }
    }

    fn rotate_right(&mut self) -> Orientation {
        use Orientation::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug)]
struct SuitLabMap {
    guard_position: (usize, usize),
    guard_orientation: Orientation,
    map: grid::Grid<char>,
    guard_history: Vec<(usize, usize)>,
    guard_unique_position_orientation_history: HashSet<(usize, usize, Orientation)>,
}

impl SuitLabMap {
    fn get_n_guard_positions(&self) -> usize {
        self.guard_history
            .iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .count()
    }

    fn get_annotated_map(&self) -> Grid<char> {
        let mut map = self.map.clone();

        // annotate the places the guard has been with an X
        for (index, value) in map.indexed_iter_mut() {
            if self.guard_history.contains(&index) {
                *value = 'X';
            }
        }
        map
    }

    fn step(&mut self) -> bool {
        let mut next_position = self.guard_position;

        let (dr, dc) = self.guard_orientation.step_direction();

        let new_row = next_position.0 as isize + dr;
        let new_col = next_position.1 as isize + dc;

        if new_row < 0
            || new_col < 0
            || new_row as usize >= self.map.rows()
            || new_col as usize >= self.map.cols()
        {
            return true;
        }

        next_position.0 = new_row as usize;
        next_position.1 = new_col as usize;

        match self.map.get(next_position.0, next_position.1) {
            Some('.') => {
                self.guard_position = next_position;
                self.guard_history.push(next_position);
            }
            Some('#') => {
                self.guard_orientation = self.guard_orientation.rotate_right();
                println!(
                    "the guard encountered an obstacle, and is now facing {:?}",
                    self.guard_orientation
                );
            }
            _ => {
                panic!("Unexpected character in the map");
            }
        }
        false
    }
}

impl FromStr for SuitLabMap {
    type Err = SuitLabMapError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(SuitLabMapError::EmptyInput);
        }

        let map_length: usize = s.lines().next().unwrap().chars().count();

        let mut map = Grid::from_vec(s.lines().flat_map(|l| l.chars()).collect(), map_length);

        // search for the guard, denoted by ^
        let mut guard_position = None;
        for ((row, col), value) in map.indexed_iter_mut() {
            if *value == '^' {
                guard_position = Some((row, col));
                *value = '.';
                break;
            }
        }

        let mut guard_unique_position_orientation_history: HashSet<(usize, usize, Orientation)> =
            HashSet::new();

        if let Some(position) = guard_position {
            guard_unique_position_orientation_history.insert((
                position.0,
                position.1,
                Orientation::Up,
            ));
            Ok(SuitLabMap {
                guard_position: position,
                guard_orientation: Orientation::Up,
                map,
                guard_history: vec![position],
                guard_unique_position_orientation_history,
            })
        } else {
            Err(SuitLabMapError::GuardNotFound)
        }
    }
}

#[derive(Debug)]
enum SuitLabMapError {
    EmptyInput,
    GuardNotFound,
}

impl fmt::Display for SuitLabMapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SuitLabMapError::EmptyInput => write!(f, "Input is empty"),
            SuitLabMapError::GuardNotFound => write!(f, "Guard not found in the map"),
        }
    }
}

impl Error for SuitLabMapError {}
