mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    let solvers = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
    ];

    for (day, solve_part) in solvers.iter().enumerate() {
        println!("Day {}", day + 1);

        let part1 = solve_part(1).unwrap_or_else(|| panic!("Cannot solve day {} part 1", day + 1));
        println!("\tPart 1: {}", part1);

        let part2 = solve_part(2).unwrap_or_else(|| panic!("Cannot solve day {} part 2", day + 1));
        println!("\tPart 2: {}", part2);
    }
}
