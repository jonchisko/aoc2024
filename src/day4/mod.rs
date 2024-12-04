// Dont have time to make this pretty
use crate::traits::Solution;

pub struct Day4 {
    pattern: Vec<char>,
    grid_data: Grid,
}

struct Grid {
    data: Vec<char>,
    height: usize,
    width: usize,
}

impl Grid {
    fn new(data: Vec<char>, height: usize, width: usize) -> Grid {
        Grid {
            data,
            height,
            width,
        }
    }

    fn number_of_elements(&self) -> usize {
        self.data.len()
    }

    //fn get_char_with_linear_index
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            pattern: "XMAS".to_string().chars().collect(),
            grid_data: Self::process_input(),
        }
    }

    fn process_input() -> Grid {
        let file_path = "./inputs/day4.txt";
        let command_data =
            std::io::read_to_string(std::fs::File::open(file_path).unwrap()).unwrap();

        let rows = command_data.split('\n').collect::<Vec<&str>>();
        let height: usize = rows.len();
        let width: usize = rows[0].trim().len();
        let grid = rows
            .iter()
            .flat_map(|element| element.trim().chars())
            .collect::<Vec<char>>();

        Grid::new(grid, height, width)
    }

    fn find_ne(x: usize, y: usize, grid_width: usize, grid_height: usize, pattern: Vec<char>, grid: &[char]) -> bool{
        let pattern_delta = pattern.len()/2;
        if x + pattern_delta < grid_width 
        && (y as i32 - pattern_delta as i32) >= 0
        && (x as i32 - pattern_delta as i32) >= 0
        && y + pattern_delta < grid_height {
            let mut found: bool = true;
            for index in (-1)..=(pattern_delta as i32) {
                if grid[(y as i32 - index) as usize * grid_width + (x as i32 + index) as usize] != pattern[(index + pattern_delta as i32) as usize] {
                    found = false;
                    break;
                }
            }

            return found;
        }

        false
    }

    fn find_se(x: usize, y: usize, grid_width: usize, grid_height: usize, pattern: Vec<char>, grid: &[char]) -> bool{
        let pattern_delta = pattern.len()/2;
        if x + pattern_delta < grid_width 
        && (y as i32 - pattern_delta as i32) >= 0
        && (x as i32 - pattern_delta as i32) >= 0
        && y + pattern_delta < grid_height {
            let mut found: bool = true;
            for index in (-1)..=(pattern_delta as i32) {
                if grid[(y as i32 + index) as usize * grid_width + (x as i32 + index) as usize] != pattern[(index + pattern_delta as i32) as usize] {
                    found = false;
                    break;
                }
            }

            return found;
        }

        false
    }
}

impl Solution for Day4 {
    fn solve1(&self) {
        let mut number_of_occurences: usize = 0;
        let pattern_delta = self.pattern.len() - 1;

        for i in 0..self.grid_data.data.len() {
            let x = i % self.grid_data.width;
            let y = i / self.grid_data.width;

            //  East
            if x + pattern_delta < self.grid_data.width {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[y * self.grid_data.width + (x + index)] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }

            //  South East
            if x + pattern_delta < self.grid_data.width && y + pattern_delta < self.grid_data.height {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[(y + index) * self.grid_data.width + (x + index)] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }

            //  South
            if y + pattern_delta < self.grid_data.height {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[(y + index) * self.grid_data.width + x] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }

            //  South West
            if (x as i32 - pattern_delta as i32) >= 0 && y + pattern_delta < self.grid_data.height {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[(y + index) * self.grid_data.width + (x - index)] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }

            //  West
            if (x as i32 - pattern_delta as i32) >= 0 {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[y * self.grid_data.width + (x - index)] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }

            //  North West
            if (x as i32 - pattern_delta as i32) >= 0 && (y as i32 - pattern_delta as i32) >= 0 {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[(y - index) * self.grid_data.width + (x - index)] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }

            //  North
            if (y as i32 - pattern_delta as i32) >= 0 {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[(y - index) * self.grid_data.width + x] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }

            //  North East
            if x + pattern_delta < self.grid_data.width && (y as i32 - pattern_delta as i32) >= 0 {
                let mut found: bool = true;
                for index in 0..self.pattern.len() {
                    if self.grid_data.data[(y - index) * self.grid_data.width + (x + index)] != self.pattern[index] {
                        found = false;
                        break;
                    }
                }

                if found {
                    number_of_occurences += 1;
                }
            }
        }

        println!("{}", number_of_occurences);
    }

    fn solve2(&self) {
        let mut number_of_occurences: usize = 0;
        let pattern_delta = 3 - 1;

        for i in 0..self.grid_data.data.len() {
            let x = i % self.grid_data.width;
            let y = i / self.grid_data.width;
            
            if self.grid_data.data[i] != 'A' {
                continue;
            }

            if (Self::find_ne(x, y, self.grid_data.width, self.grid_data.height, "MAS".chars().collect(), &self.grid_data.data[..])
            || Self::find_ne(x, y, self.grid_data.width, self.grid_data.height, "SAM".chars().collect(), &self.grid_data.data[..]))
            && (Self::find_se(x, y, self.grid_data.width, self.grid_data.height, "MAS".chars().collect(), &self.grid_data.data[..])
            || Self::find_se(x, y, self.grid_data.width, self.grid_data.height, "SAM".chars().collect(), &self.grid_data.data[..]))
             
             {
                number_of_occurences += 1;
            }
        }

        println!("{}", number_of_occurences);
    }
}
