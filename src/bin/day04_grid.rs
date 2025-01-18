use std::char;

use advent_of_code_2024::file_reader;
use grid::*;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),  // N
    (-1, 1),  // NE
    (0, 1),   // E
    (1, 1),   // SE
    (1, 0),   // S
    (1, -1),  // SW
    (0, -1),  // W
    (-1, -1), // NW
];

fn main() {
    let puzzle_input = file_reader::get_input("src/inputs/input_day04.txt");
    let grid_len: usize = puzzle_input.lines().next().unwrap().chars().count();

    let grid: Grid<char> = Grid::from_vec(
        puzzle_input.lines().flat_map(|l| l.chars()).collect(),
        grid_len,
    );

    let search_word: &str = "XMAS";
    let first_char = search_word.chars().next().unwrap();

    let problem_a: u32 = grid
        .indexed_iter()
        .filter(|&(_, &c)| c == first_char)
        .map(|((row, col), _)| search_in_directions(&grid, (row, col), search_word))
        .sum();

    println!("I found {problem_a} occurrences of the word {search_word}");

    let problem_b: usize = grid
        .indexed_iter()
        .filter(|&(_, &c)| c == 'A')
        .filter(|&(index, _)| search_x_mas(&grid, index))
        .count();

    println!("I found {problem_b} occurrences of the word MAS");
}

fn search_in_directions(grid: &Grid<char>, start: (usize, usize), word: &str) -> u32 {
    let mut matches_found = 0;

    for &(dr, dc) in &DIRECTIONS {
        let mut cursor = start.clone();
        let mut match_found = true;

        for char in word.chars() {
            match grid.get(cursor.0, cursor.1) {
                None => {
                    match_found = false;
                    break;
                }
                Some(&grid_char) => {
                    if grid_char != char {
                        match_found = false;
                        break;
                    }
                }
            }

            cursor.0 = (cursor.0 as isize + dr) as usize;
            cursor.1 = (cursor.1 as isize + dc) as usize;
        }
        if match_found {
            matches_found += 1;
        }
    }

    matches_found
}

fn search_x_mas(grid: &Grid<char>, index: (usize, usize)) -> bool {
    if grid.get(index.0, index.1) != Some(&'A') {
        return false;
    }

    // check if the characters to the bottem left
    // and top right are m and s

    let diag_1: Vec<Option<&char>> = vec![
        grid.get((index.0 as isize - 1) as usize, index.1 + 1),
        grid.get(index.0 + 1, (index.1 as isize - 1) as usize),
    ];

    let diag_2: Vec<Option<&char>> = vec![
        grid.get(index.0 + 1, index.1 + 1),
        grid.get(
            (index.0 as isize - 1) as usize,
            (index.1 as isize - 1) as usize,
        ),
    ];

    //check if both diagonals spell MAS
    // forward or backward

    if diag_1 == vec![Some(&'M'), Some(&'S')]
        || diag_1 == vec![Some(&'S'), Some(&'M')]
        || diag_2 == vec![Some(&'M'), Some(&'S')]
        || diag_2 == vec![Some(&'S'), Some(&'M')]
    {
        return true;
    }

    false
}
