use std::collections::HashMap;

use crate::traits;

pub struct Day1 {
    coordinates: Vec<u32>,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            coordinates: Self::process_data(),
        }
    }

    fn process_data() -> Vec<u32> {
        let file_path = "./inputs/day1.txt";
        let coordinates_raw =
            std::io::read_to_string(std::fs::File::open(file_path).unwrap()).unwrap();

        let coordinates = coordinates_raw
            .split(&[' ', '\n'][..])
            .filter(|coordinate| !coordinate.is_empty())
            .map(|coordinate| {
                coordinate
                    .trim()
                    .parse::<u32>()
                    .expect("The list should only hold valid u32 values.")
            })
            .collect::<Vec<u32>>();

        coordinates
    }
}

impl traits::Solution for Day1 {
    fn solve1(&self) {
        let mut coordinate_1 = self.coordinates.iter().step_by(2).collect::<Vec<&u32>>();
        let mut coordinate_2 = self
            .coordinates
            .iter()
            .skip(1)
            .step_by(2)
            .collect::<Vec<&u32>>();

        coordinate_1.sort();
        coordinate_2.sort();

        let sum: u32 = coordinate_1
            .iter()
            .zip(coordinate_2.iter())
            .map(|(x, y)| x.abs_diff(**y))
            .sum();

        println!("{}", sum);
    }

    fn solve2(&self) {
        let coordinate_1 = self.coordinates.iter().step_by(2).collect::<Vec<&u32>>();
        let coordinate_2 = self
            .coordinates
            .iter()
            .skip(1)
            .step_by(2)
            .collect::<Vec<&u32>>();

        let mut counted_list_2: HashMap<u32, u32> = HashMap::new();

        for element in coordinate_2 {
            *counted_list_2.entry(*element).or_insert(0) += 1;
        }

        let mut sum: u32 = 0;
        for element in coordinate_1 {
            sum += counted_list_2.get(element).unwrap_or(&0) * *element;
        }

        println!("{}", sum);
    }
}
