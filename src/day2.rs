use std::fs;

pub fn read_input(filepath: &str) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(filepath).expect("Should be able to read file");

    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse().expect("number parsing"))
                .collect()
        })
        .collect()
}

fn check_safety(report: &Vec<&i32>) -> bool {
    let sign = (report[1] - report[0]).signum();
    if sign == 0 {
        return false;
    }

    let mut safe = true;
    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != sign {
            safe = false;
        }
    }
    safe
}

pub fn part1(input: &Vec<Vec<i32>>) {
    let mut safe_reports = 0;
    for report in input.iter() {
        if check_safety(&report.iter().map(|x| x).collect()) {
            safe_reports += 1
        }
    }

    println!("Safe reports: {safe_reports}");
}

fn test_report(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let report_mod: Vec<&i32> = report[..i].iter().chain(&report[i + 1..]).collect();
        if check_safety(&report_mod) {
            return true;
        }
    }
    return false;
}

pub fn part2(input: &Vec<Vec<i32>>) {
    let mut safe_reports: i32 = 0;

    for report in input.iter() {
        if test_report(report) {
            safe_reports += 1;
        }
    }

    println!("Safe reports: {safe_reports}");
}
