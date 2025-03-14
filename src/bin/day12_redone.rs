use advent_of_code_2024::file_reader;
use std::collections::{HashMap, HashSet, VecDeque};

const DIRECTIONS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let input = file_reader::get_input("src/inputs/input_day12.txt");

    let garden: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(move |(j, c)| ((i as isize, j as isize), c))
        })
        .collect();

    let mut garden_copy = garden.clone();

    let mut total_fence_price: u32 = 0;

    while let Some((&plot, _)) = garden_copy.iter().next() {
        let mut fence_length = 0;
        let mut plot_area = 0;
        let mut queue = VecDeque::from([plot]);
        let mut seen = HashSet::new();
        seen.insert(plot);

        while let Some(current_plot) = queue.pop_front() {
            for dir in DIRECTIONS.iter() {
                let plot_to_check = (current_plot.0 + dir.0, current_plot.1 + dir.1);

                if garden.get(&plot_to_check) != Some(&garden[&plot]) {
                    fence_length += 1;
                } else if seen.insert(plot_to_check) {
                    queue.push_back(plot_to_check);
                }
            }
            plot_area += 1;
            garden_copy.remove(&current_plot);
        }

        total_fence_price += fence_length * plot_area;
    }

    println!("The total fence price is {total_fence_price}");
}
