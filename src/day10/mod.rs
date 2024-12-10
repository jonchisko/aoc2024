use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
};

use crate::traits::Solution;

type Point = (usize, usize);

pub struct Day10 {
    map: Vec<i8>,
    width: usize,
    height: usize,
}

impl Day10 {
    pub fn new() -> Day10 {
        let (map, width, height) = Self::process_input();
        Day10 { map, width, height }
    }

    fn process_input() -> (Vec<i8>, usize, usize) {
        let data = std::io::read_to_string(File::open("./inputs/day10.txt").unwrap()).unwrap();
        let rows: Vec<&str> = data
            .split('\n')
            .map(|row| row.trim())
            .filter(|row| !row.is_empty())
            .collect();
        let width = rows.first().unwrap().len();
        let height = rows.len();

        let map = rows
            .iter()
            .flat_map(|element| element.trim().chars())
            .map(|element| element.to_digit(10).unwrap() as i8)
            .collect::<Vec<i8>>();

        (map, width, height)
    }

    fn from_linear_to_xy(&self, linear: usize) -> Point {
        (linear % self.width, linear / self.width)
    }

    fn from_xy_to_linear(&self, xy: Point) -> usize {
        xy.1 * self.width + xy.0
    }

    fn get_peaks(&self) -> Vec<Point> {
        let mut peaks = vec![];

        for linear_index in 0..self.map.len() {
            if self.map[linear_index] == 9 {
                let xy = self.from_linear_to_xy(linear_index);
                peaks.push(xy);
            }
        }

        peaks
    }

    fn get_children(&self, xy: Point) -> Vec<Point> {
        let north = (xy.0 as i32, xy.1 as i32 - 1);
        let east = (xy.0 as i32 + 1, xy.1 as i32);
        let south = (xy.0 as i32, xy.1 as i32 + 1);
        let west = (xy.0 as i32 - 1, xy.1 as i32);
        let potential_children = vec![north, east, south, west];

        potential_children
            .iter()
            .filter(|point| self.point_ok(&xy, point))
            .map(|point| (point.0 as usize, point.1 as usize))
            .collect()
    }

    fn point_ok(&self, parent_xy: &Point, xy: &(i32, i32)) -> bool {
        xy.0 >= 0
            && xy.1 >= 0
            && (xy.0 as usize) < self.width
            && (xy.1 as usize) < self.height
            && self.map[self.from_xy_to_linear((xy.0 as usize, xy.1 as usize))] + 1
                == self.map[self.from_xy_to_linear(*parent_xy)]
    }

    fn run_dfs(&self, peaks: &[Point]) -> HashMap<Point, HashSet<Point>> {
        let mut trailhead_points = HashMap::new();
        for &peak in peaks {
            let mut queue = VecDeque::from([peak]);

            while !queue.is_empty() {
                let current_node = queue.pop_front().unwrap();
                if self.map
                    [self.from_xy_to_linear((current_node.0 as usize, current_node.1 as usize))]
                    == 0
                {
                    trailhead_points
                        .entry(current_node)
                        .or_insert(HashSet::from([peak]))
                        .insert(peak);
                }
                for child in self.get_children(current_node) {
                    queue.push_front(child);
                }
            }
        }

        trailhead_points
    }

    fn run_dfs2(&self, peaks: &[Point]) -> HashMap<Point, i32> {
        let mut trailhead_points = HashMap::new();
        for &peak in peaks {
            let mut queue = VecDeque::from([peak]);

            while !queue.is_empty() {
                let current_node = queue.pop_front().unwrap();
                if self.map
                    [self.from_xy_to_linear((current_node.0 as usize, current_node.1 as usize))]
                    == 0
                {
                    *trailhead_points.entry(current_node).or_insert(0) += 1;
                }
                for child in self.get_children(current_node) {
                    queue.push_front(child);
                }
            }
        }

        trailhead_points
    }
}

impl Solution for Day10 {
    fn solve1(&self) {
        let peaks = self.get_peaks();
        let trailhead_scores = self.run_dfs(&peaks[..]);
        let mut total_score: i32 = 0;

        for key in trailhead_scores.keys() {
            total_score += trailhead_scores.get(key).unwrap().len() as i32;
        }

        println!("Day 10.1 {}", total_score);
    }

    fn solve2(&self) {
        let peaks = self.get_peaks();
        let trailhead_scores = self.run_dfs2(&peaks[..]);
        let mut total_score: i32 = 0;

        for key in trailhead_scores.keys() {
            total_score += *trailhead_scores.get(key).unwrap();
        }

        println!("Day 10.1 {}", total_score);
    }
}
