use crate::utils;

fn is_valid_path(values: &Vec<i32>) -> bool {
    let mut is_valid = true;
    let mut direction_of_increase = 1; // 1 for increase, -1 for decrease

    for i in 0..values.len() - 1 {
        if i == 0 {
            direction_of_increase = if values[i] < values[i + 1] { 1 } else { -1 };
        }
        let diff = values[i + 1] - values[i];
        let abs = diff.abs();

        // increase or decrease length can be at least 1 or at most 3
        if direction_of_increase == 1 {
            if diff < 0 {
                is_valid = false;
                break;
            }
        } else {
            if values[i] < values[i + 1] {
                is_valid = false;
                break;
            }
        }

        if abs < 1 || abs > 3 {
            is_valid = false;
            break;
        }
    }

    is_valid
}

fn day2_part1(content: &str) -> i32 {
    let mut answer = 0;

    let lines = content.split(utils::NEW_LINE).collect::<Vec<&str>>();
    for line in lines {
        let values = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        if is_valid_path(&values) {
            answer += 1;
        }
    }

    answer
}

fn find_valid_path(values: &Vec<i32>, idx: usize, path: &mut Vec<i32>) -> bool {
    if idx >= values.len() {
        // we can only remove 0 or 1 element from total length
        if path.len() >= values.len() - 1 {
            return is_valid_path(&path);
        }
        return false;
    }

    // Try including current element
    path.push(values[idx]);
    if find_valid_path(values, idx + 1, path) {
        return true;
    }

    // backtrack to the previous state and try skipping the current element
    path.pop();

    return find_valid_path(values, idx + 1, path);
}

fn day2_part2(content: &str) -> i32 {
    let lines = content.split(utils::NEW_LINE).collect::<Vec<&str>>();
    let mut answer = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let values: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x: &str| x.parse().ok())
            .collect();

        if is_valid_path(&values) {
            answer += 1;
            continue;
        }

        if find_valid_path(&values, 0, &mut vec![]) {
            answer += 1;
        }
    }

    answer
}

pub(crate) fn day2(content: &str) -> (i32, i32) {
    (day2_part1(content), day2_part2(content))
}
