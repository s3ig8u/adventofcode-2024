use regex::Regex;

fn find_multiplications(content: &str) -> Vec<(usize, i32)> {
    static REGEX: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap());

    REGEX
        .captures_iter(content)
        .map(|m| {
            (
                m.get(0).unwrap().start(),
                m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn day3_part1(content: &str) -> i32 {
    find_multiplications(content).iter().map(|(_, r)| r).sum()
}

fn day3_part2(content: &str) -> i32 {
    let muls = find_multiplications(content);
    let dont_positions: Vec<_> = content.match_indices("don't()").map(|(i, _)| i).collect();
    let do_positions: Vec<_> = content.match_indices("do()").map(|(i, _)| i).collect();

    muls.iter()
        .filter_map(|(pos, result)| {
            let latest_dont = dont_positions.iter().rev().find(|&&i| i < *pos).copied();
            let latest_do = do_positions.iter().rev().find(|&&i| i < *pos).copied();

            match (latest_dont, latest_do) {
                (Some(dont_pos), Some(do_pos)) => {
                    // if there is don't and do we need to compare last positions
                    if do_pos > dont_pos {
                        Some(result)
                    } else {
                        None
                    }
                }
                (Some(_), None) => None, // if there is a dont but not do, mul operation is disabled
                (None, Some(_)) => Some(result), // if there is a do but not dont, mul operation is enabled
                (None, None) => Some(result),    // mul operation is enabled by default
            }
        })
        .sum()
}

pub(crate) fn day3(content: &str) -> (i32, i32) {
    (day3_part1(content), day3_part2(content))
}
