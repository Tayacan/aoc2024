use std::{fs, iter::zip};

pub fn read_input(filepath: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(filepath).expect("Should have been able to read the file");

    let lines = input.lines();

    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in lines {
        let numbers_as_strings: Vec<&str> = line.split_whitespace().collect();
        if numbers_as_strings.len() != 2 {
            println!("Skipping invalid line '{line}'");
            continue;
        }

        let n1: i32 = numbers_as_strings[0].trim().parse().expect("Not a number");
        let n2: i32 = numbers_as_strings[1].trim().parse().expect("Not a number");

        v1.push(n1);
        v2.push(n2);
    }

    return (v1, v2);
}

pub fn part1(input: (Vec<i32>, Vec<i32>)) {
    let mut v1;
    let mut v2;
    (v1, v2) = input;

    v1.sort();
    v2.sort();

    let total: i32 = zip(v1, v2).map(|(a, b)| (a - b).abs()).sum();

    println!("Total: {total}");
}

pub fn part2(input: (Vec<i32>, Vec<i32>)) {
    let v1;
    let v2;
    (v1, v2) = input;

    let mut total: i32 = 0;
    for n in v1 {
        let count: i32 = v2
            .iter()
            .filter(|&m| *m == n)
            .count()
            .try_into()
            .expect("Type conversion usize -> i32");
        total += n * count;
    }

    println!("Total: {total}");
}
