use std::{collections::HashMap, fs::File};

use crate::traits::Solution;

pub struct Day11 {
    stones: Vec<i64>,
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {
            stones: Self::process_input(),
        }
    }

    fn process_input() -> Vec<i64> {
        let stones = std::io::read_to_string(File::open("./inputs/day11.txt").unwrap()).unwrap();
        stones
            .trim()
            .split_ascii_whitespace()
            .map(|element| element.parse::<i64>().unwrap())
            .collect()
    }

    fn solve1(&self, blinks: i32) -> Vec<i64> {
        let mut current = self.stones.clone();
        let mut next = vec![];

        let mut cur_ptr = &mut current;
        let mut next_ptr = &mut next;

        for _ in 0..blinks {
            for i in 0..cur_ptr.len() {
                let rule_result = Rules::get_rule(cur_ptr[i]);
                // println!("Result {:?}", rule_result);
                match rule_result {
                    Rules::RuleOne(val) => next_ptr.push(val),
                    Rules::RuleEven(val1, val2) => {
                        next_ptr.push(val1);
                        next_ptr.push(val2);
                    }
                    Rules::RuleMultiply(val) => next_ptr.push(val),
                }
            }
            let tmp_ptr = cur_ptr;
            cur_ptr = next_ptr;
            next_ptr = tmp_ptr;
            next_ptr.clear();
            /* current = next;
            next = vec![]; */
        }

        if current.len() != 0 {
            current
        } else {
            next
        }
    }

    fn solve2(&self, blinks: i32) -> i64 {
        let mut memory = HashMap::new();
        let mut sizes = Vec::with_capacity(self.stones.len());

        for stone in &self.stones {
            sizes.push(Self::recursive_size(*stone, blinks, &mut memory));
        }

        println!("End sizes: {:?}", sizes);

        sizes.iter().sum()
    }

    fn recursive_size(number: i64, blinks: i32, memory: &mut HashMap<(i32, i64), i64>) -> i64 {
        if let Some(size) = memory.get(&(blinks, number)) {
            return *size;
        }

        let rule = Rules::get_rule(number);
        let size;

        if blinks == 1 {
            size = match rule {
                Rules::RuleOne(_) => 1,
                Rules::RuleEven(_, _) => 2,
                Rules::RuleMultiply(_) => 1,
            };
        } else {
            size = match rule {
                Rules::RuleOne(a) => Self::recursive_size(a, blinks - 1, memory),
                Rules::RuleEven(a, b) => {
                    Self::recursive_size(a, blinks - 1, memory)
                        + Self::recursive_size(b, blinks - 1, memory)
                }
                Rules::RuleMultiply(a) => Self::recursive_size(a, blinks - 1, memory),
            }
        }

        memory.insert((blinks, number), size);

        size
    }
}

#[derive(Debug, Copy, Clone)]
enum Rules {
    RuleOne(i64),
    RuleEven(i64, i64),
    RuleMultiply(i64),
}

impl Rules {
    fn get_rule(stone: i64) -> Rules {
        if stone == 0 {
            return Rules::RuleOne(1);
        }

        let digits = Self::number_of_digits(stone);
        if digits % 2 == 0 {
            let left_right = Self::divide_digits(stone, digits);
            return Rules::RuleEven(left_right.0, left_right.1);
        }

        return Rules::RuleMultiply(stone * 2024);
    }

    fn number_of_digits(mut number: i64) -> i64 {
        let mut number_of_digits = 0;
        // covered by other rule, but still
        if number == 0 {
            return 1;
        }

        while number > 0 {
            number_of_digits += 1;
            number /= 10;
        }

        number_of_digits
    }

    fn divide_digits(mut number: i64, digits: i64) -> (i64, i64) {
        let mut right = 0;

        let mut power = 1;
        for _ in 0..(digits / 2) {
            right += (number % 10) * power;
            power *= 10;
            number /= 10;
        }

        (number, right)
    }
}

impl Solution for Day11 {
    fn solve1(&self) {
        let blinks = 25i32;
        println!("Input: {:?}", self.stones);
        let solution = self.solve1(blinks);
        println!("Result: {:?}", solution.len());
    }

    fn solve2(&self) {
        let blinks = 75i32;
        println!("Input: {:?}", self.stones);
        let solution = self.solve2(blinks);
        println!("Result: {:?}", solution);
    }
}
