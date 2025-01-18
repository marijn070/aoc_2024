use advent_of_code_2024::file_reader;
use std::{collections::HashMap, str};

// Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are. Pair up the smallest number in the left list with the smallest number in the right list, then the second-smallest left number with the second-smallest right number, and so on.
// Within each pair, figure out how far apart the two numbers are; you'll need to add up all of those distances. For example, if you pair up a 3 from the left list with a 7 from the right list, the distance apart is 4; if you pair up a 9 with a 3, the distance apart is 6.
// In the example list above, the pairs and distances would be as follows:

//     The smallest number in the left list is 1, and the smallest number in the right list is 3. The distance between them is 2.
//     The second-smallest number in the left list is 2, and the second-smallest number in the right list is another 3. The distance between them is 1.
//     The third-smallest number in both lists is 3, so the distance between them is 0.
//     The next numbers to pair up are 3 and 4, a distance of 1.
//     The fifth-smallest numbers in each list are 3 and 5, a distance of 2.
//     Finally, the largest number in the left list is 4, while the largest number in the right list is 9; these are a distance 5 apart.

// To find the total distance between the left list and the right list, add up the distances between all of the pairs you found. In the example above, this is 2 + 1 + 0 + 1 + 2 + 5, a total distance of 11!

// Your actual left and right lists contain many location IDs. What is the total distance between your lists?

// fn get_input(input_path: &str) -> String {
//     let data = fs::read_to_string(input_path).expect("Unable to read file");
//     data
// }

fn get_lists(data_string: String) -> (Vec<u32>, Vec<u32>) {
    data_string
        .lines()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<_>>()
        })
        .fold((vec![], vec![]), |(mut left, mut right), line| {
            left.push(line[0]);
            right.push(line[1]);
            (left, right)
        })
}

fn get_total_difference(list1: &Vec<u32>, list2: &Vec<u32>) -> Result<u32, &'static str> {
    if list1.len() != list2.len() {
        return Err("Lists must be the same length");
    }

    let mut first: Vec<u32> = list1.clone();
    let mut second: Vec<u32> = list2.clone();

    first.sort_unstable();
    second.sort_unstable();

    let difference: u32 = first
        .into_iter()
        .zip(second)
        .map(|(a, b)| a.abs_diff(b))
        .sum();

    Ok(difference)
}

fn get_similarity_score(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    // we need to iterate over the elements in the left list,
    // looking up the amount of times they appear in the right list
    // Will sorting the list make this faster? I guess we can calculate a table
    // of occurrences for the right list and a table of occurrences in the left list?

    let mut instance_counts_right: HashMap<u32, u32> = HashMap::new();
    for &x in right.iter() {
        *instance_counts_right.entry(x).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for &x in left.iter() {
        if let Some(&count) = instance_counts_right.get(&x) {
            similarity_score += count * x;
        }
    }

    similarity_score
}

fn main() {
    let data = file_reader::get_input("src/inputs/input_day01.txt");

    let (left, right) = get_lists(data);

    let total_difference = match get_total_difference(&left, &right) {
        Ok(diff) => diff,
        Err(e) => panic!("Error: {}", e),
    };

    println!("The total difference is {total_difference}");

    let similarity_score = get_similarity_score(&left, &right);

    println!("The similarity score is {similarity_score}");
}
