mod day1;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let day = 1;

    // time the execution
    let start = std::time::Instant::now();

    let content = utils::read_input_file(format!("../input/day{}.txt", day).as_str());

    // pass content by reference instead of cloning it
    let result_part1 = day1::day1_part1(&content);
    let result_part2 = day1::day1_part2(&content);

    println!("Day {} result part 1: {}", day, result_part1);
    println!("Day {} result part 2: {}", day, result_part2);
    println!(
        "Execution time for day {}: {:?}ms",
        day,
        start.elapsed().as_millis()
    );

    Ok(())
}
