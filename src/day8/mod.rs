use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use crate::traits::Solution;

type Point = (i32, i32);

pub struct Day8 {
    antenna_map: HashMap<char, Vec<Point>>,
    map_dim: Point,
}

impl Day8 {
    pub fn new() -> Day8 {
        let input_data = Self::process_input();

        Day8 {
            antenna_map: input_data.1,
            map_dim: input_data.0,
        }
    }

    fn process_input() -> (Point, HashMap<char, Vec<Point>>) {
        let data = std::io::read_to_string(File::open("./inputs/day8.txt").unwrap()).unwrap();

        let data = data
            .split('\n')
            .filter(|row| !row.is_empty())
            .map(|row| row.trim().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let map_dim = (data.first().unwrap().len() as i32, data.len() as i32);
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

        for y in 0..map_dim.1 {
            for x in 0..map_dim.0 {
                let sign = data[y as usize][x as usize];
                if sign == '.' {
                    continue;
                }
                antennas.entry(sign).or_insert(vec![(x, y)]).push((x, y));
            }
        }

        (map_dim, antennas)
    }
}

impl Solution for Day8 {
    fn solve1(&self) {
        let mut unique_antinodes: HashSet<Point> = HashSet::new();

        for antenna_type in self.antenna_map.keys() {
            let locations = self.antenna_map.get(&antenna_type).unwrap();
            for a in locations {
                for b in locations {
                    if a == b {
                        continue;
                    }

                    let delta_a = (b.0 - a.0, b.1 - a.1);
                    let possible_antinode = (a.0 - delta_a.0, a.1 - delta_a.1);
                    if possible_antinode.0 >= 0
                        && possible_antinode.0 < self.map_dim.0
                        && possible_antinode.1 >= 0
                        && possible_antinode.1 < self.map_dim.1
                    {
                        /* println!(
                            "Antenna A: {:?} | Antenta B: {:?} -> AntiNode: {:?}",
                            a, b, possible_antinode
                        ); */

                        unique_antinodes.insert(possible_antinode);
                    }
                }
            }
        }

        //println!("{:?}", unique_antinodes);
        println!("Day 8.1 : {}", unique_antinodes.len());
    }

    fn solve2(&self) {
        let mut unique_antinodes: HashSet<Point> = HashSet::new();

        for antenna_type in self.antenna_map.keys() {
            let locations = self.antenna_map.get(&antenna_type).unwrap();
            for a in locations {
                unique_antinodes.insert(*a);
                for b in locations {
                    if a == b {
                        continue;
                    }

                    let delta_a = (b.0 - a.0, b.1 - a.1);
                    let mut possible_antinode = (a.0 - delta_a.0, a.1 - delta_a.1);

                    while possible_antinode.0 >= 0
                        && possible_antinode.0 < self.map_dim.0
                        && possible_antinode.1 >= 0
                        && possible_antinode.1 < self.map_dim.1
                    {
                        /* println!(
                            "Antenna A: {:?} | Antenta B: {:?} -> AntiNode: {:?}",
                            a, b, possible_antinode
                        ); */

                        unique_antinodes.insert(possible_antinode);
                        possible_antinode = (
                            possible_antinode.0 - delta_a.0,
                            possible_antinode.1 - delta_a.1,
                        );
                    }
                }
            }
        }

        //println!("{:?}", unique_antinodes);
        println!("Day 8.2: {}", unique_antinodes.len());
    }
}
