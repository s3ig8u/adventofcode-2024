fn expand_to_blocks(lengths: &[usize]) -> Vec<i32> {
    let total_length: usize = lengths.iter().sum();
    let mut blocks = vec![-1; total_length];
    let mut pos = 0;
    let mut file_id = 0;

    for (i, &length) in lengths.iter().enumerate() {
        for _ in 0..length {
            if i % 2 == 0 {
                blocks[pos] = file_id;
                pos += 1;
            } else {
                pos += 1;
            }
        }
        if i % 2 == 0 {
            file_id += 1;
        }
    }

    blocks
}

fn find_rightmost_file_end(blocks: &[i32]) -> Option<usize> {
    for i in (0..blocks.len()).rev() {
        if blocks[i] != -1 {
            let file_id = blocks[i];
            let mut start = i;
            while start > 0 && blocks[start - 1] == file_id {
                start -= 1;
            }
            return Some(start);
        }
    }
    None
}

fn compress_disk(mut blocks: Vec<i32>) -> Vec<i32> {
    loop {
        if let Some(file_start) = find_rightmost_file_end(&blocks) {
            if let Some(free_pos) = blocks.iter().position(|&x| x == -1) {
                if free_pos >= file_start {
                    break;
                }

                blocks[free_pos] = blocks[file_start];
                blocks[file_start] = -1;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    blocks
}

fn calculate_checksum(blocks: &[i32]) -> i64 {
    blocks.iter().enumerate().filter(|(_, &id)| id != -1).map(|(pos, &id)| pos as i64 * id as i64).sum()
}

fn day9_part1(content: &str) -> i64 {
    let lengths: Vec<usize> = content.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let blocks = expand_to_blocks(&lengths);
    let compressed = compress_disk(blocks);

    calculate_checksum(&compressed)
}

#[derive(Debug)]
struct File {
    id: i32,
    start: usize,
    length: usize,
}

fn find_files(blocks: &[i32]) -> Vec<File> {
    let mut files = Vec::new();
    let mut i = 0;

    while i < blocks.len() {
        if blocks[i] != -1 {
            let id = blocks[i];
            let start = i;
            while i < blocks.len() && blocks[i] == id {
                i += 1;
            }
            files.push(File { id, start, length: i - start });
        } else {
            i += 1;
        }
    }

    files.sort_by_key(|f| -f.id);
    files
}

fn find_leftmost_space(blocks: &[i32], needed_length: usize, before_pos: usize) -> Option<usize> {
    let mut i = 0;
    while i < before_pos {
        if blocks[i] == -1 {
            let mut space_len = 0;
            let start = i;
            while i < before_pos && blocks[i] == -1 {
                space_len += 1;
                i += 1;
            }
            if space_len >= needed_length {
                return Some(start);
            }
        } else {
            i += 1;
        }
    }
    None
}

fn compress_disk_whole_files(mut blocks: Vec<i32>) -> Vec<i32> {
    let files = find_files(&blocks);

    for file in files {
        if let Some(new_pos) = find_leftmost_space(&blocks, file.length, file.start) {
            for i in 0..file.length {
                blocks[new_pos + i] = file.id;
                blocks[file.start + i] = -1;
            }
        }
    }

    blocks
}

fn day9_part2(content: &str) -> i64 {
    let lengths: Vec<usize> = content.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let blocks = expand_to_blocks(&lengths);
    let compressed = compress_disk_whole_files(blocks);
    calculate_checksum(&compressed)
}

pub fn day9(content: &str) -> (i64, i64) {
    (day9_part1(content), day9_part2(content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_test_data() {
        assert_eq!(day9_part1(include_str!("../../../input/day9_test.txt")), 1928);
    }

    #[test]
    fn test_part1() {
        assert_eq!(day9_part1(include_str!("../../../input/day9.txt")), 6259790630969);
    }

    #[test]
    fn test_part2() {
        assert_eq!(day9_part2(include_str!("../../../input/day9.txt")), 6289564433984);
    }
}
