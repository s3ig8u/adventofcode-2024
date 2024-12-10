fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

pub fn day8(input: &str) -> (usize, usize) {
    (part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../../../input/day8_test.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 0);
    }
}
