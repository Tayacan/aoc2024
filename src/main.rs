use std::env;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: Result<i32, _> = args[1].parse();
    match n {
        Err(_) => println!("Usage: aoc2025.exe <day>"),
        Ok(1) => {
            println!("--- DAY 1 ---");
            let day1inp = day1::read_input("input/day1.txt");
            day1::part1(day1inp.clone());
            day1::part2(day1inp);
            println!();
        }
        Ok(2) => {
            println!("--- DAY 2 ---");
            let day2inp = day2::read_input("input/day2.txt");
            day2::part1(&day2inp);
            day2::part2(&day2inp);
            println!();
        }
        Ok(3) => {
            println!("--- DAY 3 ---");
            day3::part1("input/day3.txt");
            day3::part2("input/day3.txt");
            println!();
        }
        Ok(4) => {
            println!("--- DAY 4 ---");
            let day4inp = day4::read_input("input/day4.txt");
            day4::part1(&day4inp);
            day4::part2(&day4inp);
            println!();
        }
        Ok(5) => {
            println!("--- DAY 5 ---");
            let mut day5inp = day5::read_input("input/day5.txt");
            day5::both_parts(&mut day5inp);
            println!();
        }
        Ok(6) => {
            println!("--- DAY 6 ---");
            let day6inp = day6::read_input("input/day6.txt");
            day6::part1(&day6inp);
            day6::part2(&day6inp);
            println!();
        }
        Ok(_) => {
            println!("Invalid day")
        }
    }
}
