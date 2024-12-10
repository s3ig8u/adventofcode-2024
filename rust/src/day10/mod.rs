use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    i: i32,
    j: i32,
}
type Grid = Vec<Vec<i64>>;

fn parse_grid(content: &str) -> (Grid, Vec<(usize, usize)>) {
    let mut head_pos = Vec::new();
    let grid: Vec<Vec<i64>> = content
        .lines()
        .map(|line| line.chars().map(|c| if c == '.' { -1 } else { c.to_digit(10).unwrap() as i64 }).collect())
        .collect();

    grid.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, c)| {
            if *c == 0 {
                head_pos.push((i, j));
            }
        });
    });

    (grid, head_pos)
}

fn dfs_all_paths(
    grid: &Grid,
    pos: Pos,
    visited: &mut HashMap<Pos, Pos>,
    path: &mut Vec<Pos>,
    reached_nines: &mut HashSet<Pos>,
) -> i64 {
    if path.len() == 10 {
        reached_nines.insert(path[path.len() - 1]);
        return 1;
    }

    let mut paths = 0;
    let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    for dir in dirs {
        let new_pos = Pos { i: pos.i + dir.0, j: pos.j + dir.1 };
        if new_pos.i < 0 || new_pos.i >= grid.len() as i32 || new_pos.j < 0 || new_pos.j >= grid[0].len() as i32 {
            continue;
        }
        if visited.contains_key(&new_pos) {
            continue;
        }
        let next_val = grid[new_pos.i as usize][new_pos.j as usize];
        if next_val == -1 {
            continue;
        }
        let last_visited_val = grid[path[path.len() - 1].i as usize][path[path.len() - 1].j as usize];
        if next_val == last_visited_val + 1 {
            visited.insert(new_pos, pos);
            path.push(new_pos);
            paths += dfs_all_paths(grid, new_pos, visited, path, reached_nines);
            path.pop();
            visited.remove(&new_pos);
        }
    }

    paths
}

fn solve(content: &str) -> (i64, i64) {
    let (grid, head_pos) = parse_grid(content);

    let mut total_paths = 0;
    let mut total_nines = 0;
    for pos in head_pos {
        let mut path = Vec::new();
        let mut visited = HashMap::new();
        let mut reached_nines = HashSet::new();
        let start_pos = Pos { i: pos.0 as i32, j: pos.1 as i32 };

        path.push(start_pos);
        visited.insert(start_pos, start_pos);
        total_paths += dfs_all_paths(&grid, start_pos, &mut visited, &mut path, &mut reached_nines);
        total_nines += reached_nines.len() as i64;
    }

    (total_nines, total_paths)
}

pub fn day10(content: &str) -> (i64, i64) {
    solve(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const TEST_INPUT_2: &str = "\
..90..9
...1.98
...2..7
6543456
765.987
876....
987....";

    const TEST_INPUT_3: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1_input1() {
        assert_eq!(solve(TEST_INPUT_1), (2, 1));
    }

    #[test]
    fn test_part1_input2() {
        assert_eq!(solve(TEST_INPUT_2), (4, 1));
    }

    #[test]
    fn test_part1_input3() {
        assert_eq!(solve(TEST_INPUT_3), (36, 1));
    }

    #[test]
    fn test_part1_input4() {
        assert_eq!(solve(include_str!("../../../input/day10.txt")), (593, 1192));
    }
}
