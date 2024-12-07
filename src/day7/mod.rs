use std::fs::File;

use crate::traits::Solution;

pub struct Day7 {
    calculations: Vec<Vec<i64>>,
}

impl Day7 {
    pub fn new() -> Day7 {
        Day7 {
            calculations: Self::process_input(),
        }
    }

    fn process_input() -> Vec<Vec<i64>> {
        let data = std::io::read_to_string(File::open("./inputs/day7.txt").unwrap()).unwrap();

        let calculations = data
            .split('\n')
            .filter(|row| !row.is_empty())
            .map(|row| {
                row.split_ascii_whitespace()
                    .map(|number| {
                        number
                            .strip_suffix(":")
                            .unwrap_or(number)
                            .trim()
                            .parse::<i64>()
                            .expect(&format!("Number but was {}", number)[..])
                    })
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();

        calculations
    }

    fn check_sum_recursive(
        operands: &[i64],
        position: usize,
        calculation: i64,
        target: i64,
    ) -> bool {
        if position >= operands.len() {
            return calculation == target;
        }

        let value = operands[position];

        return Self::check_sum_recursive(operands, position + 1, calculation + value, target)
            || Self::check_sum_recursive(operands, position + 1, calculation * value, target);
    }

    fn check_sum_recursive2(
        operands: &[i64],
        position: usize,
        calculation: i64,
        target: i64,
    ) -> bool {
        if position >= operands.len() {
            return calculation == target;
        }

        let value = operands[position];

        return Self::check_sum_recursive2(operands, position + 1, calculation + value, target)
            || Self::check_sum_recursive2(operands, position + 1, calculation * value, target)
            || Self::check_sum_recursive2(
                operands,
                position + 1,
                Self::concat_numbers(calculation, value),
                target,
            );
    }

    fn concat_numbers(a: i64, b: i64) -> i64 {
        // you could count the power of b
        // then offset a by that power
        // sum a + b
        // but we are gona string it
        (a.to_string() + &b.to_string()).parse::<i64>().unwrap()
    }
}

// Solved with recursion -> could also do it with a queue and iterative approach
impl Solution for Day7 {
    fn solve1(&self) {
        let mut total_sum: i64 = 0;

        for calculation in &self.calculations {
            if Self::check_sum_recursive(&calculation[2..], 0, calculation[1], calculation[0]) {
                total_sum += calculation[0];
            }
        }

        println!("Day 7.1: {}", total_sum);
    }

    fn solve2(&self) {
        let mut total_sum: i64 = 0;

        for calculation in &self.calculations {
            if Self::check_sum_recursive2(&calculation[2..], 0, calculation[1], calculation[0]) {
                total_sum += calculation[0];
            }
        }

        println!("Day 7.1: {}", total_sum);
    }
}
