use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use advent_of_code_2024::file_reader;
use grid::*;

fn main() {
    let input = file_reader::get_input(&format!(
        "{}/src/inputs/input_day06.txt",
        env!("CARGO_MANIFEST_DIR")
    ));

    let mut suitmaplab = SuitLabMap::from_str(&input).unwrap();

    // fill up the history
    loop {
        let finished = suitmaplab.step();
        if finished == GuardStatus::Finished {
            break;
        }
    }

    let answer_a = suitmaplab.get_n_guard_positions();
    println!("the guard has visited {answer_a} unique positions");

    // for part b we need to obstruct the guard
    // we loop through all unique positions in the "normal" history, except the first one
    // and place an extra obstacle there.
    // Then we step and see if we find a loop

    let mut obstacle_placement_locations = suitmaplab
        .guard_history
        .clone()
        .iter()
        .map(|&(row, col, _)| (row, col))
        .collect::<HashSet<_>>();

    obstacle_placement_locations.remove(&suitmaplab.guard_starting_position);
    let mut loop_obstacle_locations: Vec<(usize, usize)> = vec![];

    for (row, col) in obstacle_placement_locations.iter() {
        let mut fresh_suitmaplab = SuitLabMap::from_str(&input).unwrap();
        fresh_suitmaplab.add_obstacle(*row, *col);
        loop {
            let step_result = fresh_suitmaplab.step();
            match step_result {
                GuardStatus::Loop => {
                    loop_obstacle_locations.push((*row, *col));
                    break;
                }
                GuardStatus::Finished => {
                    break;
                }
                GuardStatus::Normal => continue,
            }
        }
    }

    println!(
        "Found {} unique obstacle placement locations",
        loop_obstacle_locations.len()
    );
}

#[derive(Debug, PartialEq)]
enum GuardStatus {
    Finished,
    Loop,
    Normal,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct SuitLabMap {
    guard_starting_position: (usize, usize),
    guard_position: (usize, usize),
    guard_orientation: Orientation,
    map: grid::Grid<char>,
    // guard_history: Vec<(usize, usize)>,
    guard_history: HashSet<(usize, usize, Orientation)>,
}

impl SuitLabMap {
    fn add_obstacle(&mut self, row: usize, col: usize) {
        if let Some(cell) = self.map.get_mut(row, col) {
            *cell = '#';
        }
    }

    fn get_n_guard_positions(&self) -> usize {
        self.guard_history
            .iter()
            .map(|&(row, col, _)| (row, col))
            .collect::<HashSet<_>>()
            .len()
    }

    fn get_annotated_map(&self) -> Grid<char> {
        let mut map = self.map.clone();

        // annotate the places the guard has been with an X
        for (index, value) in map.indexed_iter_mut() {
            if self
                .guard_history
                .iter()
                .any(|&(col, row, _)| col == index.0 && row == index.1)
            {
                *value = 'X';
            }
        }
        map
    }

    fn step(&mut self) -> GuardStatus {
        let mut next_position = self.guard_position;

        let (dr, dc) = self.guard_orientation.step_direction();

        let new_row = next_position.0 as isize + dr;
        let new_col = next_position.1 as isize + dc;

        if new_row < 0
            || new_col < 0
            || new_row as usize >= self.map.rows()
            || new_col as usize >= self.map.cols()
        {
            return GuardStatus::Finished;
        }

        next_position.0 = new_row as usize;
        next_position.1 = new_col as usize;

        match self.map.get(next_position.0, next_position.1) {
            Some('.') => {
                self.guard_position = next_position;
                if self.guard_history.insert((
                    next_position.0,
                    next_position.1,
                    self.guard_orientation,
                )) {
                    GuardStatus::Normal
                } else {
                    GuardStatus::Loop
                }
            }
            Some('#') => {
                self.guard_orientation = self.guard_orientation.rotate_right();
                GuardStatus::Normal
            }
            _ => {
                panic!("Unexpected character in the map");
            }
        }
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
                guard_starting_position: position,
                guard_position: position,
                guard_orientation: Orientation::Up,
                map,
                guard_history: {
                    let mut set = HashSet::new();
                    set.insert((position.0, position.1, Orientation::Up));
                    set
                },
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
