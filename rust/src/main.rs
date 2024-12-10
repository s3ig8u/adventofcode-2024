mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

fn print_result<T: std::fmt::Display>(day: i32, result: (T, T), elapsed: std::time::Duration) {
    println!("Day {} - Part 1: {}", day, result.0);
    println!("Day {} - Part 2: {}", day, result.1);
    println!("Execution time for day {}: {:?}ms", day, elapsed.as_millis());
}

fn print_divider(day: i32) {
    println!("########################## Day {} ##########################", day);
}

fn read_input_file(day: i32) -> String {
    utils::read_input_file(format!("../input/day{}.txt", day).as_str())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_divider(1);
    let start = std::time::Instant::now();
    let content = read_input_file(1);
    let result1 = day1::day1(&content);
    print_result(1, result1, start.elapsed());

    print_divider(2);
    let start = std::time::Instant::now();
    let content = read_input_file(2);
    let result2 = day2::day2(&content);
    print_result(2, result2, start.elapsed());

    print_divider(3);
    let start = std::time::Instant::now();
    let content = read_input_file(3);
    let result3 = day3::day3(&content);
    print_result(3, result3, start.elapsed());

    print_divider(4);
    let start = std::time::Instant::now();
    let content = read_input_file(4);
    let result4 = day4::day4(&content);
    print_result(4, result4, start.elapsed());

    print_divider(5);
    let start = std::time::Instant::now();
    let content = read_input_file(5);
    let result5 = day5::day5(&content);
    print_result(5, result5, start.elapsed());

    // too slow for now
    // print_divider(6);
    // let start = std::time::Instant::now();
    // let content = read_input_file(6);
    // let result6 = day6::day6(&content);
    // print_result(6, result6, start.elapsed());

    print_divider(7);
    let start = std::time::Instant::now();
    let content = read_input_file(7);
    let result7 = day7::day7(&content);
    print_result(7, result7, start.elapsed());

    print_divider(8);
    let start = std::time::Instant::now();
    let content = read_input_file(8);
    let result8 = day8::day8(&content);
    print_result(8, result8, start.elapsed());

    print_divider(9);
    let start = std::time::Instant::now();
    let content = read_input_file(9);
    let result9 = day9::day9(&content);
    print_result(9, result9, start.elapsed());

    Ok(())
}
