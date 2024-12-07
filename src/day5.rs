use std::fs;

struct Rule {
    first: i32,
    last: i32,
}

pub struct PuzzleInput {
    rules: Vec<Rule>,
    pagelines: Vec<Vec<i32>>,
}

pub fn read_input(filepath: &str) -> PuzzleInput {
    let contents = fs::read_to_string(filepath).unwrap();
    let contents_proper_newlines = contents.replace("\r", "");
    let (rule_lines, page_lists) = contents_proper_newlines.split_once("\n\n").unwrap();

    let mut rules: Vec<Rule> = Vec::new();

    for rule_line in rule_lines.lines() {
        let (f, l) = rule_line.split_once("|").unwrap();
        rules.push(Rule {
            first: f.parse().unwrap(),
            last: l.parse().unwrap(),
        });
    }
    let pagelines: Vec<Vec<i32>> = page_lists
        .lines()
        .map(|line| line.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();

    PuzzleInput {
        rules: rules,
        pagelines: pagelines,
    }
}

fn find_deps(page: i32, rules: &Vec<Rule>) -> Vec<i32> {
    rules
        .iter()
        .filter(|r| r.last == page)
        .map(|r| r.first)
        .collect()
}

fn middle_element(line: &Vec<i32>) -> i32 {
    line[line.len() / 2]
}

fn line_ok(line: &Vec<i32>, rules: &Vec<Rule>) -> bool {
    (0..line.len()).all(|i| {
        let deps = find_deps(line[i], rules);
        deps.iter().all(|dep| !line[i + 1..].contains(dep))
    })
}

fn fix_line(line: &mut Vec<i32>, rules: &Vec<Rule>) {
    while !line_ok(line, rules) {
        for i in 0..line.len() {
            let deps = find_deps(line[i], rules);
            for dep in deps {
                if line[i..].contains(&dep) {
                    line.retain(|x| *x != dep);
                    line.insert(i, dep);
                }
            }
        }
    }
}

pub fn part1(input: &mut PuzzleInput) {
    let mut total_p1 = 0;
    let mut total_p2 = 0;
    for line in &mut input.pagelines {
        let line_ok = line_ok(line, &input.rules);

        if line_ok {
            total_p1 += middle_element(line);
        } else {
            fix_line(line, &input.rules);
            total_p2 += middle_element(line);
        }
    }
    println!("Total p1: {total_p1}");
    println!("Total p2: {total_p2}");
}
