use crate::traits::Solution;

struct Day3 {
    command_data: String,
    pattern: String,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            command_data: Day3::process_input(),
            pattern: "mul(X, X)".to_string(),
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
    fn solve1(&self) {
        let data_chars = self.command_data.chars().collect::<Vec<char>>();
        let pattern_chars = self.pattern.chars().collect::<Vec<char>>();

        let mut index: usize = 0;

        while index < data_chars.len() {
            let mut pattern_index: usize = 0;

            while pattern_index < pattern_chars.len() {
                if data_chars[index + pattern_index] != pattern_chars[pattern_index] {
                    index = index + pattern_index; // we might be checking one 'm' too many in this approach
                    break;
                }
                // TODO: when special X character, read digits
                // When pattern reading completed successfully -> store values to be multiplied in a vector?
                pattern_index += 1;
            }

            index += 1;
        }
    }

    fn solve2(&self) {
        todo!()
    }
}
