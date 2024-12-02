// This solution is garbage, pardon my french. Try to find to improve it from the n^2 case and all the copying etc.

use crate::traits::Solution;

pub struct Day2 {
    reports: Vec<Vec<u8>>,
}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {
            reports: Day2::process_input(),
        }
    }

    fn process_input() -> Vec<Vec<u8>> {
        let file_path = "./inputs/day2.txt";
        let reports_raw = std::io::read_to_string(std::fs::File::open(file_path).unwrap()).unwrap();

        let reports = reports_raw
            .split('\n')
            .map(|report| {
                report
                    .split(' ')
                    .filter(|report_value| !report_value.is_empty())
                    .map(|report_value| {
                        report_value
                            .trim()
                            .parse::<u8>()
                            .expect("The list should only contain valid u16 values.")
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        reports
    }

    pub fn deltas_valid(deltas: Vec<i32>) -> bool {
        let mut sign: Option<i32> = None;
        let mut valid: bool = true;
        for delta in deltas {
            if let None = sign {
                sign = Some(delta);
            }

            if let Some(value) = sign {
                if value < 0 {
                    if !(delta <= -1 && delta >= -3) {
                        valid = false;
                        break;
                    }
                } else if value > 0 {
                    if !(delta >= 1 && delta <= 3) {
                        valid = false;
                        break;
                    }
                } else {
                    valid = false;
                    break;
                }
            }
        }

        valid
    }
}

impl Solution for Day2 {
    fn solve1(&self) {
        let valid_count: i32 = self
            .reports
            .iter()
            .map(|report| {
                let deltas = report
                    .iter()
                    .zip(report.iter().skip(1))
                    .map(|pair| *pair.1 as i32 - *pair.0 as i32)
                    .collect::<Vec<i32>>();

                Day2::deltas_valid(deltas)
            })
            .fold(0, |acc, element| if element { acc + 1 } else { acc });

        println!("{}", valid_count);
    }

    fn solve2(&self) {
        let valid_count: i32 = self
            .reports
            .iter()
            .map(|report| {
                let mut valid_configuration: Vec<bool> = vec![];

                for skip_index in 0..report.len() {
                    let fixed_report = report
                        .iter()
                        .enumerate()
                        .filter(|index_pair| index_pair.0 != skip_index)
                        .map(|index_pair| index_pair.1);

                    let deltas = fixed_report
                        .clone()
                        .zip(fixed_report.skip(1))
                        .map(|pair| *pair.1 as i32 - *pair.0 as i32)
                        .collect::<Vec<i32>>();

                    valid_configuration.push(Day2::deltas_valid(deltas));
                }

                valid_configuration.iter().any(|valid| *valid)
            })
            .fold(0, |acc, element| if element { acc + 1 } else { acc });

        println!("{}", valid_count);
    }
}
