use std::collections::HashSet;
use std::fs;

pub fn read_input(filepath: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filepath).unwrap();
    contents.lines().map(|l| l.chars().collect()).collect()
}

fn get_guard_position(grid: &Vec<Vec<char>>) -> (i32, i32) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                return (x.try_into().unwrap(), y.try_into().unwrap());
            }
        }
    }
    return (-1, -1);
}

fn next_direction(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        other => {
            println!("Bad direction: {other:?}");
            other
        }
    }
}

fn get_at(grid: &Vec<Vec<char>>, (x, y): (i32, i32)) -> char {
    grid[y as usize][x as usize]
}

fn add_dir((x, y): (i32, i32), (dx, dy): (i32, i32)) -> (i32, i32) {
    (x + dx, y + dy)
}

fn is_in_grid(pos: (i32, i32), width: i32, height: i32) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

fn move_in_direction(
    pos: (i32, i32),
    dir: (i32, i32),
    grid: &Vec<Vec<char>>,
    positions: &mut HashSet<(i32, i32)>,
    extra_obstacle: Option<(i32, i32)>,
) -> ((i32, i32), bool) {
    let mut current_pos = pos;
    let mut next_pos = add_dir(current_pos, dir);

    let width: i32 = grid[0].len().try_into().unwrap();
    let height: i32 = grid.len().try_into().unwrap();

    while is_in_grid(next_pos, width, height) && get_at(grid, next_pos) != '#' {
        match extra_obstacle {
            Some(obstacle) => {
                if next_pos == obstacle {
                    break;
                }
            }
            None => (),
        }
        positions.insert(next_pos);
        current_pos = next_pos;
        next_pos = add_dir(current_pos, dir);
    }

    return (current_pos, is_in_grid(next_pos, width, height));
}

fn get_visited_positions(grid: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let mut guard_position = get_guard_position(grid);
    let mut direction = (0, -1);
    let mut still_in_grid = true;
    let mut all_positions: HashSet<(i32, i32)> = HashSet::new();
    all_positions.insert(guard_position);

    while still_in_grid {
        let (new_guard_pos, in_grid) =
            move_in_direction(guard_position, direction, grid, &mut all_positions, None);
        guard_position = new_guard_pos;
        still_in_grid = in_grid;
        direction = next_direction(direction);
    }

    all_positions
}

pub fn part1(grid: &Vec<Vec<char>>) {
    let all_positions = get_visited_positions(grid);
    let visited = all_positions.len();
    println!("Positions visited: {visited}");
}

#[derive(PartialEq, Eq, Hash)]
struct MoveInfo {
    pos: (i32, i32),
    dir: (i32, i32),
}

fn test_obstacle_position(grid: &Vec<Vec<char>>, pos: (i32, i32)) -> bool {
    let mut guard_position = get_guard_position(grid);
    let mut direction = (0, -1);
    let mut still_in_grid = true;
    let mut all_positions: HashSet<(i32, i32)> = HashSet::new();
    all_positions.insert(guard_position);
    let mut all_move_info: HashSet<MoveInfo> = HashSet::new();

    // println!("Trying to put an obstacle at {pos:?}");
    while still_in_grid {
        let (new_guard_pos, in_grid) = move_in_direction(
            guard_position,
            direction,
            grid,
            &mut all_positions,
            Some(pos),
        );
        guard_position = new_guard_pos;
        still_in_grid = in_grid;
        direction = next_direction(direction);

        let move_info = MoveInfo {
            pos: guard_position,
            dir: direction,
        };

        if all_move_info.contains(&move_info) {
            return true;
        }

        all_move_info.insert(move_info);
    }
    false
}

pub fn part2(grid: &Vec<Vec<char>>) {
    let visited_positions = get_visited_positions(grid);
    let guard_start_position = get_guard_position(grid);
    let mut valid_positions = 0;

    for position in visited_positions {
        if position == guard_start_position {
            continue;
        }
        if test_obstacle_position(grid, position) {
            valid_positions += 1;
        }
    }
    println!("Possible positions: {valid_positions}");
}
