use std::env;

fn main() {
    unsafe{
        env::set_var("RUST_BACKTRACE", "1");
    }
    //aoc_2025::day01::day01("../inputs/day1.txt");
    //aoc_2025::day02::day02("../inputs/day2.txt");
    //aoc_2025::day03::day03("../inputs/day3.txt");
    //aoc_2025::day04::day04("../inputs/day4.txt");
    aoc_2025::day05::day05("../inputs/day5.txt");
}
