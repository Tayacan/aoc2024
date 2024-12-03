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

pub fn part1(input: &Vec<Vec<i32>>) {
    let mut safe_reports = 0;
    for report in input.iter() {
        let sign = (report[1] - report[0]).signum();
        if sign == 0 {
            continue;
        }

        let mut safe = true;
        for i in 0..report.len() - 1 {
            let diff = report[i + 1] - report[i];
            if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != sign {
                safe = false;
            }
        }
        if !safe {
            continue;
        }

        safe_reports += 1;
    }

    println!("Safe reports: {safe_reports}");
}

fn test_report(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let report_mod: Vec<&i32> = report[..i].iter().chain(&report[i + 1..]).collect();

        let sign = (report_mod[1] - report_mod[0]).signum();
        if sign == 0 {
            continue;
        }

        let mut safe = true;
        for i in 0..report_mod.len() - 1 {
            let diff = report_mod[i + 1] - report_mod[i];
            if diff.abs() > 3 || diff.abs() == 0 || diff.signum() != sign {
                safe = false;
            }
        }
        if safe {
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
