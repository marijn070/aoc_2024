use advent_of_code_2024::file_reader;
use std::collections::HashSet;

fn main() {
    let input = file_reader::get_input(&format!(
        "{}/src/inputs/test_day10.txt",
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
        trailhead_ends.push(find_unique_trail_ends(&map_grid, trailhead))
    }

    let answer_a = trailhead_ends.iter().fold(0, |acc, x| acc + x.len());

    println!(
        "the trailheads have scores of {:?}, making for a total score of {}",
        trailhead_ends
            .iter()
            .map(|x| x.len())
            .collect::<Vec<usize>>(),
        answer_a
    );

    let mut trailhead_paths = vec![];

    for &trailhead in trailheads.iter() {
        let path: Vec<(usize, usize)> = vec![trailhead];
        let mut paths = HashSet::new();
        paths.insert(path);
        trailhead_paths.push(find_unique_trails(&map_grid, paths, trailhead))
    }

    let answer_b = trailhead_paths.iter().fold(0, |acc, x| acc + x.len());

    println!(
        "the trailheads have scores of {:?}, making for a total score of {}",
        trailhead_paths
            .iter()
            .map(|x| x.len())
            .collect::<Vec<usize>>(),
        answer_b
    );

    // we can also get answer a from the paths
    // let answer_a: usize = trailhead_paths
    //     .iter()
    //     .map(|h| h.iter().map(|l| l.last()).unique().count()) // count how many unique endpoints each set of trails has
    //     .sum();
}

fn find_unique_trails(
    map: &Vec<Vec<u8>>,
    paths: HashSet<Vec<(usize, usize)>>,
    coords: (usize, usize),
) -> HashSet<Vec<(usize, usize)>> {
    let map_size = (map.len(), map[0].len());
    let current_height = map[coords.0][coords.1];

    let paths_till_here: HashSet<Vec<(usize, usize)>> = paths
        .iter()
        .map(|path| {
            let mut new_path = path.clone();
            new_path.push(coords);
            new_path
        })
        .collect();
    if current_height == 9 {
        return paths_till_here;
    }

    let mut updated_paths: HashSet<Vec<(usize, usize)>> = HashSet::new();

    for adjacent_spot in get_adjacent_coords_in_bounds(coords, map_size) {
        if map[adjacent_spot.0][adjacent_spot.1] == current_height + 1 {
            for paths in find_unique_trails(map, paths_till_here.clone(), adjacent_spot) {
                updated_paths.insert(paths);
            }
        }
    }
    updated_paths
}

fn find_unique_trail_ends(map: &Vec<Vec<u8>>, coords: (usize, usize)) -> HashSet<(usize, usize)> {
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
            for trail_end in find_unique_trail_ends(map, adjacent_spot) {
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
