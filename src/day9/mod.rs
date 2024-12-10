use std::fs::File;

use crate::traits::Solution;

pub struct Day9 {
    decoded: Vec<i32>,
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            decoded: Self::process_input(),
        }
    }

    fn process_input() -> Vec<i32> {
        let data = std::io::read_to_string(File::open("./inputs/day9.txt").unwrap()).unwrap();

        let mut decoded: Vec<i32> = vec![];
        let mut block_id: i32 = 0;

        for (index, element) in data.trim().chars().enumerate() {
            let repeat = element.to_digit(10).unwrap();
            if index % 2 == 0 {
                Self::decode_element(&mut decoded, block_id, repeat as usize);
                block_id += 1;
            } else {
                Self::decode_element(&mut decoded, -1, repeat as usize);
            }
        }

        decoded
    }

    fn decode_element(decoded: &mut Vec<i32>, id: i32, size: usize) {
        for _ in 0..size {
            decoded.push(id);
        }
    }

    fn swap(decoded: &mut [i32], i: usize, j: usize) {
        let tmp: i32 = decoded[i];
        decoded[i] = decoded[j];
        decoded[j] = tmp;
    }

    fn get_empty_spaces(decoded: &[i32]) -> Vec<(usize, i32)> {
        let mut empty_spaces = vec![];

        let mut i: usize = 0;
        loop {
            if i >= decoded.len() {
                break;
            }

            if decoded[i] == -1 {
                let mut j = i;

                while j < decoded.len() && decoded[j] == -1 {
                    j += 1;
                }

                empty_spaces.push((i, (j - i) as i32));
                i = j;

                continue;
            }

            i += 1;
        }

        empty_spaces
    }

    fn get_size_of_block(decoded: &[i32], j: usize, block_id: i32) -> i32 {
        let mut start: i32 = j as i32;
        while start >= 0 && decoded[start as usize] == block_id {
            start -= 1;
        }

        j as i32 - start
    }

    fn swap_block(
        decoded: &mut [i32],
        empty_spaces: &mut [(usize, i32)],
        block_id: i32,
        block_end_index: usize,
        block_size: i32,
    ) {
        for i in 0..empty_spaces.len() {
            if empty_spaces[i].1 > 0
                && empty_spaces[i].0 < block_end_index
                && empty_spaces[i].1 > 0
                && empty_spaces[i].1 >= block_size
            {
                for new_location_index in
                    empty_spaces[i].0..(empty_spaces[i].0 + block_size as usize)
                {
                    decoded[new_location_index] = block_id;
                }

                for erase_block_index in
                    (block_end_index as i32 - (block_size - 1))..(block_end_index as i32 + 1)
                {
                    decoded[erase_block_index as usize] = -1;
                }

                empty_spaces[i].1 -= block_size;
                empty_spaces[i].0 += block_size as usize;

                break;
            }
        }
    }
}

impl Solution for Day9 {
    fn solve1(&self) {
        let mut decoded = self.decoded.clone();
        let mut i = 0;
        let mut j = decoded.len() - 1;

        loop {
            while decoded[i] != -1 {
                i += 1;
            }

            while decoded[j] == -1 {
                j -= 1;
            }

            if i >= j {
                break;
            }

            Self::swap(&mut decoded, i, j);
        }

        let mut sum: i64 = 0;
        for pos in 0..decoded.len() {
            if decoded[pos] == -1 {
                continue;
            }

            sum += (pos as i32 * decoded[pos]) as i64;
        }

        println!("Day 9.1: {}", sum);
    }

    fn solve2(&self) {
        let mut decoded = self.decoded.clone();
        let mut empty_spaces = Self::get_empty_spaces(&decoded[..]);

        let mut j = decoded.len() - 1;

        loop {
            while decoded[j] == -1 {
                j -= 1;
            }

            let block_id = decoded[j];
            let block_size = Self::get_size_of_block(&decoded, j, block_id);

            if j as i32 - block_size as i32 <= 0 {
                break;
            }
            /* println!(
                "Block id {}, block size {}, state {:?}",
                block_id, block_size, decoded
            ); */
            Self::swap_block(&mut decoded, &mut empty_spaces, block_id, j, block_size);
            j = j - block_size as usize;
        }

        let mut sum: i64 = 0;
        for pos in 0..decoded.len() {
            if decoded[pos] == -1 {
                continue;
            }

            sum += (pos as i32 * decoded[pos]) as i64;
        }

        //println!("Decoded {:?}", decoded);

        println!("Day 9.2: {}", sum);
    }
}
