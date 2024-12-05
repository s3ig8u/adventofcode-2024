use std::collections::{HashMap, VecDeque};

fn get_sequences(content: &str) -> Vec<Vec<i32>> {
    content
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn get_dependency_map_by_sequence(content: &str, sequence: &[i32]) -> HashMap<i32, Vec<i32>> {
    let mut dependency_map: HashMap<i32, Vec<i32>> = sequence.iter().map(|&num| (num, Vec::new())).collect();

    for line in content.lines().take_while(|line| !line.is_empty()) {
        let parts: Vec<i32> = line.split("|").map(|num| num.parse().unwrap()).collect();
        if sequence.contains(&parts[0]) && sequence.contains(&parts[1]) {
            dependency_map.get_mut(&parts[1]).unwrap().push(parts[0]);
        }
    }
    dependency_map
}

fn is_valid_sequence(sequence: &[i32], dependency_map: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, &current) in sequence.iter().enumerate() {
        if let Some(deps) = dependency_map.get(&current) {
            // tracking the deps of current node, e.g: current = 2, deps = [1, 3]. if 1 and 3 are in the previous numbers, then it's a valid sequence
            let previous_numbers: Vec<i32> = sequence[0..i].to_vec();
            if !deps.iter().all(|dep| previous_numbers.contains(dep)) {
                return false;
            }
        }
    }
    true
}

fn topological_sort(sequence: &[i32], content: &str) -> Vec<i32> {
    let dependency_map = get_dependency_map_by_sequence(content, sequence);
    // init in_degree map
    // e.g: { a:0, b:0 ..}
    let mut in_degree: HashMap<i32, usize> = sequence.iter().map(|&n| (n, 0)).collect();

    for deps in dependency_map.values() {
        for &node in deps {
            *in_degree.entry(node).or_default() += 1;
        }
    }

    let mut result = Vec::with_capacity(sequence.len());

    // find all nodes with no dependencies
    let mut no_deps: VecDeque<i32> = in_degree.iter().filter(|(_, &count)| count == 0).map(|(&node, _)| node).collect();

    while let Some(node) = no_deps.pop_front() {
        result.push(node);
        if let Some(deps) = dependency_map.get(&node) {
            for &dep in deps {
                let count = in_degree.get_mut(&dep).unwrap();
                *count -= 1;
                if *count == 0 {
                    no_deps.push_back(dep);
                }
            }
        }
    }

    result
}

fn day5_part1(content: &str) -> i32 {
    let sequences: Vec<Vec<i32>> = get_sequences(content);

    sequences
        .iter()
        .filter_map(|sequence| {
            let dependency_map = get_dependency_map_by_sequence(content, sequence);
            if is_valid_sequence(sequence, &dependency_map) {
                Some(sequence[sequence.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn day5_part2(content: &str) -> i32 {
    let sequences = get_sequences(content);

    sequences
        .iter()
        .filter_map(|sequence| {
            let dependency_map = get_dependency_map_by_sequence(content, sequence);
            if !is_valid_sequence(sequence, &dependency_map) {
                let correct_sequence = topological_sort(sequence, content);
                Some(correct_sequence[correct_sequence.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

pub(crate) fn day5(content: &str) -> (i32, i32) {
    (day5_part1(content), day5_part2(content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_sequence() {
        let content = "1|2\n2|3\n1|3\n\n1,2,3";
        let sequence = vec![1, 2, 3];
        let dependency_map = get_dependency_map_by_sequence(content, &sequence);
        assert!(is_valid_sequence(&sequence, &dependency_map));
    }

    #[test]
    fn test_invalid_sequence() {
        let content = "1|2\n2|3\n1|3\n\n2,1,3";
        let sequence = vec![2, 1, 3];
        let dependency_map = get_dependency_map_by_sequence(content, &sequence);
        assert!(!is_valid_sequence(&sequence, &dependency_map));
    }
}
