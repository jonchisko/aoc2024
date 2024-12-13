use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
};

use crate::traits::Solution;

type Point = (usize, usize);

pub struct Day12 {
    map: Vec<char>,
    width: usize,
    height: usize,
}

impl Day12 {
    pub fn new() -> Day12 {
        let (map, width, height) = Self::process_input();
        Day12 { map, width, height }
    }

    fn process_input() -> (Vec<char>, usize, usize) {
        let data = std::io::read_to_string(File::open("./inputs/day12.txt").unwrap()).unwrap();

        let data: Vec<&str> = data
            .trim()
            .split('\n')
            .map(|element| element.trim())
            .collect();
        let height = data.len();
        let width = data[0].len();

        (
            data.iter().flat_map(|element| element.chars()).collect(),
            width,
            height,
        )
    }

    fn linear_to_xy(&self, linear_index: usize) -> Point {
        (linear_index % self.width, linear_index / self.width)
    }

    fn xy_to_linear(&self, xy: Point) -> usize {
        self.width * xy.1 + xy.0
    }

    fn get_all_children(&self, source_point: Point) -> Vec<(i64, i64)> {
        let mut children = vec![];

        children.push((source_point.0 as i64, source_point.1 as i64 - 1));
        children.push((source_point.0 as i64 + 1, source_point.1 as i64));
        children.push((source_point.0 as i64, source_point.1 as i64 + 1));
        children.push((source_point.0 as i64 - 1, source_point.1 as i64));

        children
    }

    fn get_children(&self, current: Point) -> Vec<usize> {
        let symbol = self.map[self.xy_to_linear(current)];
        self.get_all_children(current)
            .iter()
            .filter(|neighbour| self.is_node_valid(symbol, **neighbour))
            .map(|neighbour| self.xy_to_linear((neighbour.0 as usize, neighbour.1 as usize)))
            .collect()
    }

    fn is_node_valid(&self, current: char, neighbour: (i64, i64)) -> bool {
        return self.in_bounds(neighbour)
            && self.map[self.xy_to_linear((neighbour.0 as usize, neighbour.1 as usize))]
                == current;
    }

    fn in_bounds(&self, neighbour: (i64, i64)) -> bool {
        neighbour.0 >= 0
            && neighbour.1 >= 0
            && (neighbour.0 as usize) < self.width
            && (neighbour.1 as usize) < self.height
    }

    fn dfs(
        &self,
        start: usize,
        id: usize,
        visited: &mut HashSet<usize>,
        areas: &mut HashMap<usize, (usize, usize)>,
    ) {
        let mut queue: VecDeque<usize> = VecDeque::from([start]);

        while queue.len() >= 1 {
            let current_lin_index = queue.pop_front().unwrap();
            if visited.contains(&current_lin_index) {
                continue;
            }
            visited.insert(current_lin_index);
            let children = self.get_children(self.linear_to_xy(current_lin_index));
            let val = areas.entry(id).or_insert((0, 0));
            val.0 += 1;
            val.1 += 4 - children.len();
            // get border sides of current
            // increase values if exist or insert start vals
            // get children
            // push them to queue
            for child in children {
                queue.push_front(child);
            }
        }
    }
}

impl Solution for Day12 {
    fn solve1(&self) {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut areas: HashMap<usize, (usize, usize)> = HashMap::new();
        let mut group_id = 0usize;
        for linear_index in 0..self.map.len() {
            //let xy =

            if visited.contains(&linear_index) {
                continue;
            }
            self.dfs(linear_index, group_id, &mut visited, &mut areas);
            group_id += 1;
        }

        let mut sum = 0;

        for key in areas.keys() {
            let area = areas.get(key).unwrap();
            sum += area.0 * area.1;
        }

        println!("{}", sum);
    }

    fn solve2(&self) {
        todo!()
    }
}
