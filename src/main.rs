use traits::Solution;

pub mod day1;
pub mod traits;

fn main() {
    let day1 = day1::Day1::new();
    solve(day1);
}

fn solve<T>(solution: T)
where
    T: Solution,
{
    solution.solve1();
    solution.solve2();
}
