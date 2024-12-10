const OPERATIONS: [char; 2] = ['+', '*'];
const OPERATIONS_2: [char; 3] = ['+', '*', '|'];

fn try_combinations(values: &[i64], ops: &[char], target: i64, current: i64, pos: usize) -> bool {
    if current > target {
        return false;
    }
    if pos == values.len() - 1 {
        return current == target;
    }

    for &op in ops {
        let next = match op {
            '+' => current + values[pos + 1],
            '*' => current * values[pos + 1],
            '|' => {
                let mut s = String::with_capacity(20);
                s.push_str(&current.to_string());
                s.push_str(&values[pos + 1].to_string());
                s.parse().unwrap()
            }
            _ => current,
        };
        if try_combinations(values, ops, target, next, pos + 1) {
            return true;
        }
    }
    false
}

fn get_vec_from_content(content: &str) -> Vec<(i64, Vec<i64>)> {
    content
        .lines()
        .filter_map(|line| {
            line.split_once(": ").map(|(key, values)| {
                let target = key.parse().unwrap();
                let values = values.split_whitespace().map(|v| v.parse().unwrap()).collect();
                (target, values)
            })
        })
        .collect()
}

fn calculate(operations: &[char], vec: &[(i64, Vec<i64>)]) -> i64 {
    vec.iter()
        .filter_map(
            |(target, values)| {
                if try_combinations(values, operations, *target, values[0], 0) {
                    Some(*target)
                } else {
                    None
                }
            },
        )
        .sum()
}

fn day7_part1(content: &str) -> i64 {
    calculate(&OPERATIONS, &get_vec_from_content(content))
}

fn day7_part2(content: &str) -> i64 {
    calculate(&OPERATIONS_2, &get_vec_from_content(content))
}

pub(crate) fn day7(content: &str) -> (i64, i64) {
    (day7_part1(content), day7_part2(content))
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_day7_part1() {
        let content = utils::read_input_file("../input/day7_test.txt");
        let result = day7_part1(&content);
        assert_eq!(result, 3749);
    }
}
