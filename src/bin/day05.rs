use advent_of_code_2024::file_reader;
use std::collections::HashMap;

fn main() {
    let input = file_reader::get_input("src/inputs/input_day05.txt");

    // the ordering page rules and the pages to produce
    // are separated by an empty line
    let (ordering_page_rules, pages_to_produce) = input.split_once("\n\n").unwrap();

    let mut comes_after: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut comes_before: HashMap<u32, Vec<u32>> = HashMap::new();

    // loop through the puzzle ordering rules
    for rule in ordering_page_rules.lines() {
        let nums: Vec<u32> = rule
            .split('|')
            .filter_map(|x| x.parse::<u32>().ok())
            .take(2)
            .collect();

        comes_after.entry(nums[0]).or_default().push(nums[1]);
        comes_before.entry(nums[1]).or_default().push(nums[0]);
    }

    let page_sequences: Vec<Vec<u32>> = pages_to_produce
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect()
        })
        .collect();

    // split the sequences into correct and incorrect ones
    let (correct_page_sequences, incorrect_page_sequences): (Vec<_>, Vec<_>) = page_sequences
        .clone()
        .into_iter()
        .partition(|sequence| check_page_sequence(sequence, &comes_before, &comes_after));

    println!(
        "Out of the {} sequences of pages, {} were correct and {} were incorrect",
        page_sequences.len(),
        correct_page_sequences.len(),
        incorrect_page_sequences.len()
    );

    let answer_a = sum_middle_page_numbers(&correct_page_sequences);
    println!("The sum of the middle page numbers is {answer_a}");

    let corrected_page_sequences: Vec<Vec<u32>> = incorrect_page_sequences
        .iter()
        .map(|sequence| correct_page_sequence(sequence, &comes_before, &comes_after))
        .collect();

    println!(
        "We just created {} corrected page sequences, the length of the incorrect vector is now {}",
        corrected_page_sequences.len(),
        incorrect_page_sequences.len()
    );

    println!("checking again with our function if the page sequences are correct");

    let all_corrected_ok: bool = corrected_page_sequences
        .iter()
        .all(|s| check_page_sequence(s, &comes_before, &comes_after));

    match all_corrected_ok {
        true => println!("according to us, they are all correct"),
        false => println!("oops, they are not correct"),
    }

    let answer_b = sum_middle_page_numbers(&corrected_page_sequences);
    println!("The answer to part b is {answer_b}");
}

fn sum_middle_page_numbers(page_sequences: &[Vec<u32>]) -> u32 {
    page_sequences.iter().map(|x| x[x.len() / 2]).sum()
}

fn check_page_sequence(
    sequence: &Vec<u32>,
    before_rules: &HashMap<u32, Vec<u32>>,
    after_rules: &HashMap<u32, Vec<u32>>,
) -> bool {
    for (i, page) in sequence.iter().enumerate() {
        if let Some(after_pages) = after_rules.get(page) {
            if sequence[..i].iter().any(|x| after_pages.contains(x)) {
                return false;
            }
        }

        if let Some(before_pages) = before_rules.get(page) {
            if sequence[i..].iter().any(|x| before_pages.contains(x)) {
                return false;
            }
        }
    }
    true
}

fn correct_page_sequence(
    sequence: &[u32],
    before_rules: &HashMap<u32, Vec<u32>>,
    after_rules: &HashMap<u32, Vec<u32>>,
) -> Vec<u32> {
    let mut old_sequence = sequence.to_vec();
    let mut corrected_sequence: Vec<u32> = vec![];

    while corrected_sequence.len() < sequence.len()
        || !check_page_sequence(&corrected_sequence, &before_rules, &after_rules)
    {
        // loop through the sequence, and see if one of the pages
        // has zero of the other pages coming after it.

        // here we find the idx for the page that must be added next
        let mut next_idx: Option<usize> = None;

        for (i, page) in old_sequence.iter().enumerate() {
            let mut other_pages = old_sequence.to_vec();
            other_pages.remove(i);
            let empty_vec = vec![];
            let before_pages = before_rules.get(page).unwrap_or(&empty_vec);
            if !other_pages.iter().any(|x| before_pages.contains(x)) {
                next_idx = Some(i);
            }
        }

        if let Some(index) = next_idx {
            corrected_sequence.push(old_sequence[index]);
            old_sequence.remove(index);
        }
    }
    corrected_sequence
}
