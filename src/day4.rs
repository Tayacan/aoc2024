use std::{fs, usize};

pub fn read_input(filepath: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filepath).unwrap();
    contents
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            chars
        })
        .collect()
}

// From https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }

    transposed
}

fn search_horizontal(grid: &Vec<Vec<char>>, word: &str) -> usize {
    grid.iter()
        .map(|line| {
            (0..line.len() - word.len() + 1)
                .map(|i| {
                    let matches = word.chars().enumerate().all(|(j, c)| line[i + j] == c);
                    let matches_reverse = word
                        .chars()
                        .rev()
                        .enumerate()
                        .all(|(j, c)| line[i + j] == c);
                    if matches || matches_reverse {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn search_diagonal(grid: &Vec<Vec<char>>, word: &str) -> usize {
    let mut sum = 0;
    for j in 0..grid.len() - word.len() + 1 {
        for i in 0..grid[0].len() - word.len() + 1 {
            // Right and down
            let matches_rd = word
                .chars()
                .enumerate()
                .all(|(k, c)| grid[j + k][i + k] == c);
            let matches_rd_reverse = word
                .chars()
                .rev()
                .enumerate()
                .all(|(k, c)| grid[j + k][i + k] == c);
            if matches_rd || matches_rd_reverse {
                sum += 1;
            }
            // Left and down
            let w = grid[0].len() - 1;
            let matches_ld = word
                .chars()
                .enumerate()
                .all(|(k, c)| grid[j + k][(w - i) - k] == c);
            let matches_ld_reverse = word
                .chars()
                .rev()
                .enumerate()
                .all(|(k, c)| grid[j + k][(w - i) - k] == c);
            if matches_ld || matches_ld_reverse {
                sum += 1;
            }
        }
    }
    sum
}

pub fn part1(input: &Vec<Vec<char>>) {
    let sum_h = search_horizontal(input, "XMAS");
    let sum_v = search_horizontal(&transpose(input.clone()), "XMAS");
    let sum_d = search_diagonal(input, "XMAS");

    let total = sum_h + sum_v + sum_d;
    println!("XMAS: {total}");
}

pub fn part2(input: &Vec<Vec<char>>) {
    let mut sum = 0;
    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            if input[y][x] == 'A'
                && ((input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S')
                    || (input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M'))
                && ((input[y - 1][x + 1] == 'M' && input[y + 1][x - 1] == 'S')
                    || (input[y - 1][x + 1] == 'S' && input[y + 1][x - 1] == 'M'))
            {
                sum += 1;
            }
        }
    }
    println!("X-MAS: {sum}");
}
