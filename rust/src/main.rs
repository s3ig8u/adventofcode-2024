mod day1;
mod day2;
mod utils;

fn print_result(day: i32, result: (i32, i32), elapsed: std::time::Duration) {
    println!("Day {} - Part 1: {}", day, result.0);
    println!("Day {} - Part 2: {}", day, result.1);
    println!(
        "Execution time for day {}: {:?}ms",
        day,
        elapsed.as_millis()
    );
}

fn print_divider(day: i32) {
    println!(
        "########################## Day {} ##########################",
        day
    );
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

    Ok(())
}
