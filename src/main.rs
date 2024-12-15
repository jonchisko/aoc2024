use traits::Solution;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod traits;

fn main() {
    /* let day1 = day1::Day1::new();
    solve(day1);

    let day2 = day2::Day2::new();
    solve(day2);

    let day3 = day3::Day3::new();
    solve(day3);

    let day4 = day4::Day4::new();
    solve(day4);

    let day5 = day5::Day5::new();
    solve(day5);

    let day6 = day6::Day6::new();
    solve(day6);

    let day7 = day7::Day7::new();
    solve(day7);

    let day8 = day8::Day8::new();
    solve(day8);

    let day9 = day9::Day9::new();
    solve(day9);

    let day10 = day10::Day10::new();
    solve(day10);

    let day11 = day11::Day11::new();
    solve(day11);

    let day12 = day12::Day12::new();
    solve(day12);*/

    let day13 = day13::Day13::new();
    solve(day13);
}

fn solve<T>(solution: T)
where
    T: Solution,
{
    solution.solve1();
    solution.solve2();
}
