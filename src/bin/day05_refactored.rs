use advent_of_code_2024::file_reader;
use std::collections::HashMap;

#[derive(Debug)]
struct PageRules {
    comes_after: HashMap<u32, Vec<u32>>,
    comes_before: HashMap<u32, Vec<u32>>,
}

impl PageRules {
    fn new() -> Self {
        Self {
            comes_after: HashMap::new(),
            comes_before: HashMap::new(),
        }
    }

    fn add_rule(&mut self, before: u32, after: u32) {
        self.comes_after.entry(before).or_default().push(after);
        self.comes_before.entry(after).or_default().push(before);
    }

    fn check_sequence(&self, sequence: &[u32]) -> bool {
        for (i, &page) in sequence.iter().enumerate() {
            if let Some(after_pages) = self.comes_after.get(&page) {
                if sequence[..i].iter().any(|x| after_pages.contains(x)) {
                    return false;
                }
            }

            if let Some(before_pages) = self.comes_before.get(&page) {
                if sequence[i..].iter().any(|x| before_pages.contains(x)) {
                    return false;
                }
            }
        }
        true
    }

    fn correct_sequence(&self, sequence: &[u32]) -> Vec<u32> {
        let mut old_sequence = sequence.to_vec();
        let mut corrected_sequence: Vec<u32> = vec![];

        while corrected_sequence.len() < sequence.len() || !self.check_sequence(&corrected_sequence)
        {
            let mut next_idx: Option<usize> = None;

            for (i, &page) in old_sequence.iter().enumerate() {
                let mut other_pages = old_sequence.clone();
                other_pages.remove(i);
                let empty_vec = &vec![];
                let before_pages = self.comes_before.get(&page).unwrap_or(empty_vec);
                if !other_pages.iter().any(|x| before_pages.contains(x)) {
                    next_idx = Some(i);
                    break;
                }
            }

            if let Some(index) = next_idx {
                corrected_sequence.push(old_sequence[index]);
                old_sequence.remove(index);
            }
        }
        corrected_sequence
    }
}

fn main() {
    let input = file_reader::get_input("src/inputs/input_day05.txt");

    let (ordering_page_rules, pages_to_produce) = input.split_once("\n\n").unwrap();

    let mut page_rules = PageRules::new();

    for rule in ordering_page_rules.lines() {
        let nums: Vec<u32> = rule
            .split('|')
            .filter_map(|x| x.parse::<u32>().ok())
            .take(2)
            .collect();
        page_rules.add_rule(nums[0], nums[1]);
    }

    let page_sequences: Vec<Vec<u32>> = pages_to_produce
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|x| x.parse::<u32>().ok())
                .collect()
        })
        .collect();

    let (correct_page_sequences, incorrect_page_sequences): (Vec<_>, Vec<_>) = page_sequences
        .clone()
        .into_iter()
        .partition(|sequence| page_rules.check_sequence(sequence));

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
        .map(|sequence| page_rules.correct_sequence(sequence))
        .collect();

    println!(
        "We just created {} corrected page sequences, the length of the incorrect vector is now {}",
        corrected_page_sequences.len(),
        incorrect_page_sequences.len()
    );

    println!("Checking again with our function if the page sequences are correct");

    let all_corrected_ok: bool = corrected_page_sequences
        .iter()
        .all(|s| page_rules.check_sequence(s));

    match all_corrected_ok {
        true => println!("According to us, they are all correct"),
        false => println!("Oops, they are not correct"),
    }

    let answer_b = sum_middle_page_numbers(&corrected_page_sequences);
    println!("The answer to part b is {answer_b}");
}

fn sum_middle_page_numbers(page_sequences: &[Vec<u32>]) -> u32 {
    page_sequences.iter().map(|x| x[x.len() / 2]).sum()
}
