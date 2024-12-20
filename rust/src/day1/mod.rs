#![allow(dead_code)]

use std::collections::HashMap;

// return tuple of two vectors (arrays)
fn get_columns(content: &str) -> (Vec<i32>, Vec<i32>) {
    let mut column1: Vec<i32> = Vec::with_capacity(content.lines().count()); // allocate memory for n elements
    let mut column2: Vec<i32> = Vec::with_capacity(content.lines().count()); // allocate memory for n elements

    for line in content.lines() {
        let values = line.split_whitespace().collect::<Vec<&str>>();

        let num1 = values[0].parse::<i32>().unwrap();
        let num2 = values[1].parse::<i32>().unwrap();

        column1.push(num1);
        column2.push(num2);
    }

    (column1, column2)
}
fn day1_part1(content: &str) -> i32 {
    let (mut column1, mut column2) = get_columns(content);

    column1.sort();
    column2.sort();

    let answer: i32 = column1
        .iter()
        .zip(column2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    answer
}

fn day1_part2(content: &str) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let (column1, column2) = get_columns(content);

    for &val in column2.iter() {
        *map.entry(val).or_insert(0) += 1; // insert value if not present, otherwise increment
    }

    let mut answer: i32 = 0;
    for &val in column1.iter() {
        answer += val * map.get(&val).unwrap_or(&0);
    }

    answer
}

pub(crate) fn day1(content: &str) -> (i32, i32) {
    (day1_part1(content), day1_part2(content))
}
