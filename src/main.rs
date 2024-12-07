use traits::Solution;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod traits;

fn main() {
    let day1 = day1::Day1::new();
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
}

fn solve<T>(solution: T)
where
    T: Solution,
{
    solution.solve1();
    solution.solve2();
}
