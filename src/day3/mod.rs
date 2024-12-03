use crate::traits::Solution;

pub struct Day3 {
    command_data: String,
    pattern: String,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            command_data: Day3::process_input(),
            pattern: "mul(X,X)".to_string(),
        }
    }

    fn process_input() -> String {
        let file_path = "./inputs/day3.txt";
        let command_data =
            std::io::read_to_string(std::fs::File::open(file_path).unwrap()).unwrap();

        command_data
    }
}

impl Solution for Day3 {
    // "Non" regex solution
    fn solve1(&self) {
        let mut numbers: Vec<u16> = vec![];
        let data_chars = self.command_data.chars().collect::<Vec<char>>();
        let pattern_chars = self.pattern.chars().collect::<Vec<char>>();

        let mut index: usize = 0;

        while index < data_chars.len() {
            let mut pattern_index: usize = 0;
            let mut candidate: Vec<String> = vec![];

            while pattern_index < pattern_chars.len() {
                if pattern_chars[pattern_index] == 'X' {
                    // try to read the 1-3 digit number
                    let mut number: String = String::new();
                    let mut digit_index: usize = 0;
                    loop {
                        if !data_chars[index + pattern_index + digit_index].is_numeric() {
                            if number.len() > 0 {
                                candidate.push(number);
                            }
                            break;
                        }

                        number.push(data_chars[index + pattern_index + digit_index]);
                        digit_index += 1;
                    }

                    index += digit_index-1;
                    pattern_index += 1;
                }
                
                if data_chars[index + pattern_index] != pattern_chars[pattern_index] {
                    index = if data_chars[index + pattern_index] == 'm' {
                        index + pattern_index
                    } else {
                        index + pattern_index + 1
                    };

                    break;
                }

                pattern_index += 1;
            }

            if pattern_index == pattern_chars.len() {
                if candidate.len() == 2 && candidate[0].len() <= 3 && candidate[1].len() <= 3 {
                    numbers.push(candidate[0].parse::<u16>().unwrap());
                    numbers.push(candidate[1].parse::<u16>().unwrap());
                }
                index += pattern_index;
            }
        }

        println!("{:?}", numbers.iter().step_by(2).zip(numbers.iter().skip(1).step_by(2)).map(|pair| *pair.0 as u32 * *pair.1 as u32).sum::<u32>())
    }

    fn solve2(&self) {
        todo!()
    }
}
