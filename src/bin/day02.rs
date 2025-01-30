use std::str::FromStr;

use advent_of_code_2024::file_reader;

#[derive(Debug)]
struct ReportGrid {
    grid: Vec<Vec<i32>>,
}

#[derive(Debug)]
struct ReportGridError {}

impl FromStr for ReportGrid {
    type Err = ReportGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = vec![];
        for line in s.lines() {
            let report = line
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            grid.push(report);
        }

        Ok(ReportGrid { grid })
    }
}

impl ReportGrid {
    fn check_report_safety(report: &[i32]) -> bool {
        if report.len() < 2 {
            return false;
        }

        let mut increasing = None;

        for window in report.windows(2) {
            let diff = window[1] - window[0];
            if diff.abs() < 1 || diff.abs() > 3 {
                return false;
            }

            match increasing {
                Some(inc) => {
                    if (diff > 0) != inc {
                        return false;
                    }
                }
                None => {
                    increasing = Some(diff > 0);
                }
            }
        }
        true
    }

    fn problem_a(&self) -> usize {
        self.grid
            .iter()
            .filter(|report| ReportGrid::check_report_safety(report))
            .count()
    }

    fn problem_damper(report: &[i32]) -> bool {
        for i in 0..report.len() {
            let mut damped_report = report.to_owned();
            damped_report.remove(i);

            if ReportGrid::check_report_safety(&damped_report) {
                return true;
            }
        }
        false
    }

    fn problem_b(self) -> usize {
        // we want to give the failing reports a second chance
        // by removing levels. Im gonna do this the ugly way and
        // iteratively try removing a level.

        let n_problem_damper_reports = self
            .grid
            .iter()
            .filter(|report| !ReportGrid::check_report_safety(report))
            .filter(|report| ReportGrid::problem_damper(report))
            .count();

        n_problem_damper_reports
    }
}

fn main() {
    let input = file_reader::get_input("src/inputs/input_day02.txt");
    // we need to check which lines are safe
    // we will iterate per line, and can call a recursive function or iterate over the line
    //
    // dbg!(&reports);

    let reports = ReportGrid::from_str(&input).unwrap();
    let n_safe_reports = reports.problem_a();
    let n_problem_damper_reports = reports.problem_b();

    println!("The number of safe reports is {n_safe_reports}");
    println!("The number of safe reports with problem damper is {n_problem_damper_reports}");
    println!(
        "The total number of safe reports is {}",
        n_safe_reports + n_problem_damper_reports
    );
}
