use advent_of_code_2024::file_reader;
use std::collections::HashSet;

// we need to find the number of hiking paths for each trailhead
// 1. find all the trailhead locations
// 2. initiate a tree for each trailhead (so each point along a path can have multiple child trails)
//      Probably nice to define our own tree? each node will have a height and an array of children
// 3. figure out early stopping (so in the get_child_trails function we return false if there are +1 slope paths)
//      - for all neighbors where the hight is one higher, add a child node
//      - if we find child trails, append them to the children of the current node and run that function on its children
//      - if we dont find child trails, return from the function
// 4. at the end, count the number of child nodes for each trail head
//      This can be done with a counter and a recursive function (maybe)

fn main() {
    let input = file_reader::get_input(&format!(
        "{}/src/inputs/input_day10.txt",
        env!("CARGO_MANIFEST_DIR")
    ));

    let map_grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.chars()
                .filter_map(|c| c.to_digit(10).map(|d| d as u8))
                .collect()
        })
        .collect();

    let mut trailheads: Vec<(usize, usize)> = vec![];

    map_grid.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &element)| {
            if element == 0 {
                trailheads.push((i, j));
            }
        });
    });

    let mut trailhead_ends = vec![];

    for &trailhead in trailheads.iter() {
        trailhead_ends.push(find_trail_ends(&map_grid, trailhead))
    }

    println!(
        "the trailheads have scores of {:?}, making for a total score of {}",
        trailhead_ends
            .iter()
            .map(|x| x.len())
            .collect::<Vec<usize>>(),
        trailhead_ends.iter().fold(0, |acc, x| acc + x.len())
    );
}

fn find_trail_ends(map: &Vec<Vec<u8>>, coords: (usize, usize)) -> HashSet<(usize, usize)> {
    let map_size = (map.len(), map[0].len());
    let current_height = map[coords.0][coords.1];
    let mut trail_ends = HashSet::new();
    // check if we reached the top of the trail
    if map[coords.0][coords.1] == 9 {
        trail_ends.insert(coords);
        return trail_ends;
    }

    // check adjacent spots and add children
    for adjacent_spot in get_adjacent_coords_in_bounds(coords, map_size) {
        if map[adjacent_spot.0][adjacent_spot.1] == current_height + 1 {
            for trail_end in find_trail_ends(map, adjacent_spot) {
                trail_ends.insert(trail_end);
            }
        }
    }

    trail_ends
}

fn get_adjacent_coords_in_bounds(
    coords: (usize, usize),
    map_size: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut adjacent_coords = Vec::new();
    // Define the possible directions (up, down, right, left)
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    for (dx, dy) in directions.iter() {
        let new_x = coords.0 as isize + dx;
        let new_y = coords.1 as isize + dy;

        if new_x >= 0 && new_x < map_size.0 as isize && new_y >= 0 && new_y < map_size.1 as isize {
            adjacent_coords.push((new_x as usize, new_y as usize));
        }
    }

    adjacent_coords
}
