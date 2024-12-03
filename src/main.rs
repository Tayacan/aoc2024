mod day1;
mod day2;
mod day3;

fn main() {
    println!("--- DAY 1 ---");
    let day1inp = day1::read_input("input/day1.txt");
    day1::part1(day1inp.clone());
    day1::part2(day1inp);
    println!();

    println!("--- DAY 2 ---");
    let day2inp = day2::read_input("input/day2.txt");
    day2::part1(&day2inp);
    day2::part2(&day2inp);
    println!();

    println!("--- DAY 3 ---");
    day3::part1("input/day3.txt");
    day3::part2("input/day3.txt");
    println!();
}
