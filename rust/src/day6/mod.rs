// I messed up with this one!!
use std::{
    collections::HashSet,
    io::{Error, ErrorKind},
};

const SHE: char = '^';
const POSSIBLE_MOVE: [char; 4] = ['^', '>', 'v', '<'];
fn get_lab_model(content: &str) -> Vec<Vec<char>> {
    content.lines().map(|line| line.chars().collect()).collect()
}

fn find_she_position(lab_model: &Vec<Vec<char>>) -> (i32, i32) {
    for (i, row) in lab_model.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == SHE {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("She not found");
}

fn get_next_position(current_position: (i32, i32), current_move: char, lab_model: &Vec<Vec<char>>) -> (i32, i32) {
    let (x, y) = current_position;
    match current_move {
        '^' => (x - 1, y),
        '>' => (x, y + 1),
        'v' => (x + 1, y),
        '<' => (x, y - 1),
        _ => panic!("Invalid move"),
    }
}

fn is_in_bounds(position: (i32, i32), lab_model: &Vec<Vec<char>>) -> bool {
    position.0 >= 0 && position.1 >= 0 && position.0 < lab_model.len() as i32 && position.1 < lab_model[0].len() as i32
}

fn is_obstacle(position: (i32, i32), lab_model: &Vec<Vec<char>>) -> bool {
    lab_model[position.0 as usize][position.1 as usize] == '#'
}

fn get_next_move(current_move: char) -> char {
    POSSIBLE_MOVE[(POSSIBLE_MOVE.iter().position(|&c| c == current_move).unwrap() + 1) % POSSIBLE_MOVE.len()]
}

fn get_total_visited_positions(lab_model: &Vec<Vec<char>>) -> Result<i32, Error> {
    let mut she_position = find_she_position(&lab_model);
    let m = lab_model.len();
    let n = lab_model[0].len();
    let mut current_move = POSSIBLE_MOVE[0];
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::with_capacity(m * n);
    let mut visited_positions_with_direction: HashSet<((i32, i32), char)> = HashSet::with_capacity(m * n);
    visited_positions.insert(she_position);
    let mut number_of_visited_positions = 1;

    while she_position.0 > 0 && she_position.1 > 0 && she_position.0 < m as i32 - 1 && she_position.1 < n as i32 - 1 {
        let next_position = get_next_position(she_position, current_move, &lab_model);

        if !is_in_bounds(next_position, &lab_model) {
            break;
        }

        if is_obstacle(next_position, &lab_model) {
            current_move = get_next_move(current_move);
        } else {
            she_position = next_position;
            if !visited_positions.contains(&she_position) {
                visited_positions.insert(she_position);
                number_of_visited_positions += 1;
                visited_positions_with_direction.insert((she_position, current_move));
            } else {
                if visited_positions_with_direction.contains(&((she_position, current_move))) {
                    return Err(Error::new(ErrorKind::Other, "Loop found"));
                }
            }
        }
    }
    Ok(number_of_visited_positions)
}

fn day6_part1(content: &str) -> i32 {
    let lab_model = get_lab_model(content);
    let result = get_total_visited_positions(&lab_model);

    match result {
        Ok(value) => value,
        Err(e) => panic!("Error: {}", e),
    }
}

fn day6_part2(content: &str) -> i32 {
    let lab_model = get_lab_model(content);
    let first_position = find_she_position(&lab_model);
    let mut she_position = find_she_position(&lab_model);
    let m = lab_model.len();
    let n = lab_model[0].len();
    let mut current_move = POSSIBLE_MOVE[0];
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::with_capacity(m * n);
    visited_positions.insert(she_position);

    while she_position.0 > 0 && she_position.1 > 0 && she_position.0 < m as i32 - 1 && she_position.1 < n as i32 - 1 {
        let next_position = get_next_position(she_position, current_move, &lab_model);

        if !is_in_bounds(next_position, &lab_model) {
            break;
        }

        if is_obstacle(next_position, &lab_model) {
            current_move = get_next_move(current_move);
        } else {
            she_position = next_position;
            if !visited_positions.contains(&she_position) {
                visited_positions.insert(she_position);
            }
        }
    }
    let mut sum = 0;
    let lab_model = get_lab_model(content);

    for pos in visited_positions.iter() {
        if *pos != first_position {
            let mut replaced_model = lab_model.clone();
            replaced_model[pos.0 as usize][pos.1 as usize] = '#';
            let result = get_total_visited_positions(&replaced_model);
            match result {
                Ok(_) => (),
                Err(_) => sum += 1,
            }
        }
    }

    sum
}

pub(crate) fn day6(content: &str) -> (i32, i32) {
    (day6_part1(content), day6_part2(content))
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_day6_part1() {
        let content = utils::read_input_file("../input/day6_test.txt");
        let result = day6_part1(&content);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_day6_part2() {
        let content = utils::read_input_file("../input/day6_test.txt");
        let result = day6_part2(&content);
        assert_eq!(result, 6);
    }
}
