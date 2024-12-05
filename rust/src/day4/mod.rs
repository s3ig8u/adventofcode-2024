#[derive(Clone, Debug)]
struct Position {
    x: i16,
    y: i16,
}

fn day4_part1(content: &str) -> i32 {
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len() as i16;
    let cols = matrix[0].len() as i16;

    const DIRECTIONS: [(i16, i16); 8] = [
        (0, 1),   // right
        (1, 0),   // down
        (0, -1),  // left
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    const SEARCH_WORD: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for x in 0..rows {
        for y in 0..cols {
            if matrix[x as usize][y as usize] != 'X' {
                continue;
            }

            for &(dx, dy) in &DIRECTIONS {
                let mut valid = true;

                for i in 1..SEARCH_WORD.len() {
                    let new_x = x + dx * i as i16;
                    let new_y = y + dy * i as i16;

                    if new_x < 0 || new_x >= rows || new_y < 0 || new_y >= cols {
                        valid = false;
                        break;
                    }

                    if matrix[new_x as usize][new_y as usize] != SEARCH_WORD[i] {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    count += 1;
                }
            }
        }
    }

    count
}

/*
   M.S
   .A.
   M.S

    - A: [a_x, a_y]
    - M: [a_x - 1, a_y - 1], [a_x + 1, a_y - 1] ,
    - S: [a_x - 1, a_y + 1], [a_x + 1, a_y + 1]
*/

fn get_pattern(a_x: i16, a_y: i16) -> Vec<Vec<(char, Position)>> {
    vec![
        // M-A-S and M-A-S
        vec![
            ('M', Position { x: a_x - 1, y: a_y - 1 }),
            ('S', Position { x: a_x + 1, y: a_y + 1 }),
            ('M', Position { x: a_x - 1, y: a_y + 1 }),
            ('S', Position { x: a_x + 1, y: a_y - 1 }),
        ],
        // M-A-S and S-A-M
        vec![
            ('M', Position { x: a_x - 1, y: a_y - 1 }),
            ('S', Position { x: a_x + 1, y: a_y + 1 }),
            ('S', Position { x: a_x - 1, y: a_y + 1 }),
            ('M', Position { x: a_x + 1, y: a_y - 1 }),
        ],
        // S-A-M and M-A-S
        vec![
            ('S', Position { x: a_x - 1, y: a_y - 1 }),
            ('M', Position { x: a_x + 1, y: a_y + 1 }),
            ('M', Position { x: a_x - 1, y: a_y + 1 }),
            ('S', Position { x: a_x + 1, y: a_y - 1 }),
        ],
        // S-A-M and S-A-M
        vec![
            ('S', Position { x: a_x - 1, y: a_y - 1 }),
            ('M', Position { x: a_x + 1, y: a_y + 1 }),
            ('S', Position { x: a_x - 1, y: a_y + 1 }),
            ('M', Position { x: a_x + 1, y: a_y - 1 }),
        ],
    ]
}

fn day4_part2(content: &str) -> i32 {
    let matrix: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let rows = matrix.len() as i16;
    let cols = matrix[0].len() as i16;
    let mut count = 0;

    for x in 1..rows - 1 {
        for y in 1..cols - 1 {
            if matrix[x as usize][y as usize] != 'A' {
                continue;
            }

            for pattern in get_pattern(x, y) {
                if pattern.iter().all(|(c, p)| {
                    p.x >= 0 && p.x < rows && p.y >= 0 && p.y < cols && matrix[p.x as usize][p.y as usize] == *c
                }) {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

pub fn day4(content: &str) -> (i32, i32) {
    (day4_part1(content), day4_part2(content))
}
