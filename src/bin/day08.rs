use advent_of_code_2024::file_reader;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = file_reader::get_input("src/inputs/input_day08.txt");

    // we need a grid, bounds checking, and iterating over pair
    // I think maybe a hashmap, with characters as the key (antenans),
    // and a vector of locations of those antennas
    // Then we can iterator over each pair, and insert the antinodes into an
    // antinodes hashset made up of tuples of (row, col)

    let grid_height = input.lines().count();
    let grid_length = input.lines().next().unwrap().chars().count();

    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            antenna_locations
                .entry(char)
                .and_modify(|locations| locations.push((i, j)))
                .or_insert(vec![(i, j)]);
        }
    }

    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut antenna_lines: Vec<AntennaLine> = vec![];

    for (_, locations) in antenna_locations.into_iter() {
        locations.iter().combinations(2).for_each(|antenna_pair| {
            get_antinodes(*antenna_pair[0], *antenna_pair[1], grid_height, grid_length)
                .into_iter()
                .for_each(|antinode| {
                    antinodes.insert(antinode);
                });

            antenna_lines.push(AntennaLine::from_points(*antenna_pair[0], *antenna_pair[1]));
        });
    }

    println!("The number of distinct antinodes is {}", antinodes.len());

    let mut antennaline_antinodes: HashSet<(usize, usize)> = HashSet::new();

    for (i, j) in (0..grid_height).cartesian_product(0..grid_length) {
        for antenna_line in antenna_lines.iter() {
            if antenna_line.on((i, j)) {
                antennaline_antinodes.insert((i, j));
                break;
            }
        }
    }

    println!(
        "the number of unique points on on a line between two antennas is {}",
        antennaline_antinodes.len()
    );

    for (i, line) in input.clone().lines().enumerate() {
        let line_str: String = line
            .chars()
            .enumerate()
            .map(|(j, x)| match antennaline_antinodes.contains(&(i, j)) {
                true => '#',
                false => x,
            })
            .collect();
        println!("{}", line_str);
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct AntennaLine {
    a: isize,
    b: isize,
    c: isize,
}

impl AntennaLine {
    pub fn on(&self, point: (usize, usize)) -> bool {
        self.a * point.0 as isize + self.b * point.1 as isize + self.c == 0
    }

    pub fn from_points(point_a: (usize, usize), point_b: (usize, usize)) -> Self {
        let a = point_b.1 as isize - point_a.1 as isize;
        let b = point_a.0 as isize - point_b.0 as isize;
        let c = point_b.0 as isize * point_a.1 as isize - point_a.0 as isize * point_b.1 as isize;
        Self { a, b, c }
    }
}

fn get_antinodes(
    point_a: (usize, usize),
    point_b: (usize, usize),
    height: usize,
    width: usize,
) -> Vec<(usize, usize)> {
    let row_diff = point_a.0 as isize - point_b.0 as isize;
    let col_diff = point_a.1 as isize - point_b.1 as isize;

    let antinode1 = (point_a.0 as isize + row_diff, point_a.1 as isize + col_diff);
    let antinode2 = (point_b.0 as isize - row_diff, point_b.1 as isize - col_diff);

    let mut antinodes = Vec::new();

    if (0..height as isize).contains(&antinode1.0) && (0..width as isize).contains(&antinode1.1) {
        antinodes.push((antinode1.0 as usize, antinode1.1 as usize));
    }
    if (0..height as isize).contains(&antinode2.0) && (0..width as isize).contains(&antinode2.1) {
        antinodes.push((antinode2.0 as usize, antinode2.1 as usize));
    }

    antinodes
}
