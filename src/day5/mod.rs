use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use crate::traits::Solution;

pub struct Day5 {
    relations: HashMap<u8, HashSet<u8>>,
    manuals: Vec<Vec<u8>>,
}

impl Day5 {
    pub fn new() -> Day5 {
        Day5 {
            relations: Self::process_input(),
            manuals: Self::process_input_manuals(),
        }
    }

    fn process_input_manuals() -> Vec<Vec<u8>> {
        let data = std::io::read_to_string(File::open("./inputs/day5.txt").unwrap()).unwrap();

        let manuals = data
            .split('\n')
            .rev()
            .skip(1)
            .take_while(|manual| {
                !manual.is_empty() && *manual != "\r"
            })
            .map(|manual| {
                manual
                    .trim()
                    .split(',')
                    .map(|element| element.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        manuals
    }

    fn process_input() -> HashMap<u8, HashSet<u8>> {
        let data = std::io::read_to_string(File::open("./inputs/day5.txt").unwrap()).unwrap();
        let mut mapping: HashMap<u8, HashSet<u8>> = HashMap::new();

        let processed_data = data
            .split("\r\n")
            .take_while(|pair| !pair.is_empty())
            .map(|pair| {
                let mut pair = pair.split('|');
                let first = pair.next().unwrap();
                let second = pair.next().unwrap();

                (
                    first.trim().parse::<u8>().unwrap(),
                    second.trim().parse::<u8>().unwrap(),
                )
            });

        for pair in processed_data {
            mapping
                .entry(pair.0)
                .and_modify(|value| {
                    value.insert(pair.1);
                })
                .or_insert(HashSet::from([pair.1]));
        }
        mapping
    }
}

impl Solution for Day5 {
    fn solve1(&self) {
        let mut sum: u32 = 0;
        for manual in &self.manuals {
            let mut is_ok = true;
            for i in 0..(manual.len() - 1) {
                for j in (i + 1)..manual.len() {
                    if !self.relations.contains_key(&manual[i])
                        || !self.relations[&manual[i]].contains(&manual[j])
                    {
                        is_ok = false;
                    }
                }
            }

            if is_ok {
                sum += manual[manual.len() / 2] as u32;
            }
        }

        // Seems to not work anymore after I had to do some modifications for windows?
        println!("Day 5, 1: {}", sum);
    }

    fn solve2(&self) {
        let mut sum: u32 = 0;
        for manual in &self.manuals {
            let relevant_rules: Vec<(&u8, &HashSet<u8>)> = self
                .relations
                .iter()
                .filter(|pair| manual.contains(pair.0))
                .collect();

            let priorities = relevant_rules
                .iter()
                .map(|pair| {
                    (
                        pair.0,
                        pair.1
                            .iter()
                            .filter(|value| manual.contains(&value))
                            .count(),
                    )
                })
                .collect::<Vec<(&u8, usize)>>();

            let mut prioritized = manual
                .iter()
                .map(|element| {
                    (
                        element,
                        priorities
                            .iter()
                            .filter(|prio| prio.0 == element)
                            .map(|prio| prio.1)
                            .next()
                            .unwrap_or(0),
                    )
                })
                .collect::<Vec<(&u8, usize)>>();

            let mut sorted = true;
            for i in 0..(prioritized.len() - 1) {
                if prioritized[i].1 < prioritized[i + 1].1 {
                    sorted = false;
                }
            }

            if sorted {
                continue;
            }

            prioritized.sort_by(|a, b| b.1.cmp(&a.1));
            sum += *prioritized[prioritized.len() / 2].0 as u32;
        }

        println!("Day 5, 2: {}", sum);
    }
}
