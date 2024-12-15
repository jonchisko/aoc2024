use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use crate::traits::Solution;

pub struct Day6 {
    world: Vec<WorldElement>,
    world_size: (usize, usize),
}

impl Day6 {
    pub fn new() -> Day6 {
        let (world_size, world) = Self::process_input();

        Day6 { world, world_size }
    }

    fn process_input() -> ((usize, usize), Vec<WorldElement>) {
        let data = std::io::read_to_string(File::open("./inputs/day6.txt").unwrap()).unwrap();

        let grid: Vec<&str> = data.split('\n').filter(|row| !row.is_empty()).collect();
        let world_size = (grid[0].trim().len(), grid.len());

        let world_elements = data
            .split('\n')
            .flat_map(|line| line.chars())
            .filter(|&element| element == '#' || element == '^' || element == '.')
            .map(|element| WorldElement::from(element))
            .collect();

        (world_size, world_elements)
    }

    fn from_x_y_to_linear(&self, x: usize, y: usize) -> usize {
        self.world_size.0 * y + x
    }

    fn from_linear_to_x_y(&self, linear_index: usize) -> (usize, usize) {
        (
            linear_index % self.world_size.0,
            linear_index / self.world_size.0,
        )
    }

    fn is_position_in_world(&self, x: i32, y: i32) -> bool {
        x < self.world_size.0 as i32 && x >= 0 && y < self.world_size.1 as i32 && y >= 0
    }

    fn next_move(
        &self,
        world: &Vec<WorldElement>,
        position: (i32, i32),
        direction: GuardDirection,
    ) -> ((i32, i32), GuardDirection) {
        let delta = direction.get_delta_position();
        let mut next = (position.0 + delta.0, position.1 + delta.1);
        let mut direction = direction;

        if !self.is_position_in_world(next.0, next.1) {
            return (next, direction);
        }

        // if one could place an obstacle also on previous paths, this would become inf loop in some case
        while world[self.from_x_y_to_linear(next.0 as usize, next.1 as usize)]
            == WorldElement::Obstacle
        {
            direction = direction.turn_direction();
            let delta = direction.get_delta_position();
            next = (position.0 + delta.0, position.1 + delta.1);
        }

        (next, direction)
    }

    fn check_in_line_if_cycle_possible(
        &self,
        position: (i32, i32),
        mut direction: GuardDirection,
        previous_positions: &HashMap<(i32, i32), Vec<GuardDirection>>,
        world: &mut Vec<WorldElement>,
        new_obstacle_pos: (i32, i32),
    ) -> bool {
        //println!("Searching at {:?}", position);
        let mut current_position = position;
        let mut previous_positions = previous_positions.clone();
        let previous_element = world
            [self.from_x_y_to_linear(new_obstacle_pos.0 as usize, new_obstacle_pos.1 as usize)];
        world[self.from_x_y_to_linear(new_obstacle_pos.0 as usize, new_obstacle_pos.1 as usize)] =
            WorldElement::Obstacle;

        loop {
            let (next, new_direction) = self.next_move(&world, current_position, direction);

            if !self.is_position_in_world(next.0, next.1) {
                world[self.from_x_y_to_linear(
                    new_obstacle_pos.0 as usize,
                    new_obstacle_pos.1 as usize,
                )] = previous_element;
                return false;
            }

            current_position = next;
            direction = new_direction;

            if let Some(previous_directions) = previous_positions.get(&current_position) {
                if previous_directions.contains(&direction) {
                    //println!("{:?}", current_position);
                    world[self.from_x_y_to_linear(
                        new_obstacle_pos.0 as usize,
                        new_obstacle_pos.1 as usize,
                    )] = previous_element;
                    return true;
                }
            }

            previous_positions
                .entry(current_position)
                .or_insert(vec![direction])
                .push(direction);
        }
    }
}

impl Solution for Day6 {
    fn solve1(&self) {
        let mut guard_position = None;
        let mut guard_direction = None;

        for i in 0..self.world.len() {
            if let WorldElement::Guard(direction) = self.world[i] {
                guard_position = Some(i);
                guard_direction = Some(direction);
            }
        }

        // Should have just used i32 everywhere ...
        let mut position = self.from_linear_to_x_y(guard_position.unwrap());
        let mut direction = guard_direction.unwrap();
        let mut previous_positions: HashSet<(usize, usize)> = HashSet::new();
        //let mut delta;
        //let mut next;
        let world = self.world.clone();

        loop {
            if !self.is_position_in_world(position.0 as i32, position.1 as i32) {
                break;
            }
            previous_positions.insert(position);

            let (next, new_direction) =
                self.next_move(&world, (position.0 as i32, position.1 as i32), direction);

            position = (next.0 as usize, next.1 as usize);
            direction = new_direction;
        }

        println!("Day 6.1: {}", previous_positions.len());
    }

    fn solve2(&self) {
        let mut total_sum: usize = 0;
        let mut possible_obstacles: HashSet<(i32, i32)> = HashSet::new();
        let mut guard_position = None;
        let mut guard_direction = None;

        for i in 0..self.world.len() {
            if let WorldElement::Guard(direction) = self.world[i] {
                guard_position = Some(i);
                guard_direction = Some(direction);
            }
        }

        // Should have just used i32 everywhere ...
        let mut position = self.from_linear_to_x_y(guard_position.unwrap());
        let mut direction = guard_direction.unwrap();
        let mut previous_positions: HashMap<(i32, i32), Vec<GuardDirection>> = HashMap::new();
        //let mut delta;
        //let mut next;
        let mut world = self.world.clone();

        loop {
            if !self.is_position_in_world(position.0 as i32, position.1 as i32) {
                break;
            }

            previous_positions
                .entry((position.0 as i32, position.1 as i32))
                .or_insert(vec![direction])
                .push(direction);

            let (next, new_direction) =
                self.next_move(&world, (position.0 as i32, position.1 as i32), direction);

            if !previous_positions.contains_key(&next) && self.is_position_in_world(next.0, next.1)
            {
                if self.check_in_line_if_cycle_possible(
                    (position.0 as i32, position.1 as i32),
                    direction.turn_direction(),
                    &previous_positions,
                    &mut world,
                    next,
                ) {
                    possible_obstacles.insert(next);
                    total_sum += 1;
                }
            }

            position = (next.0 as usize, next.1 as usize);
            direction = new_direction;
        }

        println!(
            "Day 6.2: {}, total sum: {}",
            possible_obstacles.len(),
            total_sum
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum WorldElement {
    Obstacle,
    Empty,
    Guard(GuardDirection),
}

impl From<char> for WorldElement {
    fn from(value: char) -> Self {
        match value {
            '.' => WorldElement::Empty,
            '#' => WorldElement::Obstacle,
            '^' => WorldElement::Guard(GuardDirection::North),
            _ => panic!("World element does not exist!"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum GuardDirection {
    North,
    East,
    South,
    West,
}

impl From<char> for GuardDirection {
    fn from(value: char) -> Self {
        match value {
            '^' => GuardDirection::North,
            '>' => GuardDirection::East,
            'v' => GuardDirection::South,
            '<' => GuardDirection::West,
            _ => panic!("Wrong direction!"),
        }
    }
}

impl GuardDirection {
    fn get_delta_position(&self) -> (i32, i32) {
        match self {
            &GuardDirection::North => (0, -1),
            &GuardDirection::East => (1, 0),
            &GuardDirection::South => (0, 1),
            &GuardDirection::West => (-1, 0),
        }
    }

    fn turn_direction(&self) -> Self {
        match self {
            &GuardDirection::North => GuardDirection::East,
            &GuardDirection::East => GuardDirection::South,
            &GuardDirection::South => GuardDirection::West,
            &GuardDirection::West => GuardDirection::North,
        }
    }
}
