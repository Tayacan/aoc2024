use regex::Regex;
use std::fs;

fn parse_sequence(sequence: &str) -> Vec<(i32, i32)> {
    let re = Regex::new("mul\\(([0-9][0-9]?[0-9]?),([0-9][0-9]?[0-9]?)\\)").unwrap();
    re.captures_iter(sequence)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn eval_sequence(sequence: &str) -> i32 {
    parse_sequence(sequence).iter().map(|(a, b)| a * b).sum()
}

pub fn part1(filepath: &str) {
    let contents = fs::read_to_string(filepath).expect("File read");

    let sum = eval_sequence(&contents);

    println!("Sum: {sum}");
}

pub fn part2(filepath: &str) {
    let contents = fs::read_to_string(filepath).expect("File read");

    let sequences = contents.split("do()");
    let sum: i32 = sequences
        .map(|seq| eval_sequence(seq.split("don't()").collect::<Vec<_>>()[0]))
        .sum();
    println!("Sum: {sum}");
}
