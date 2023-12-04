use std::collections::HashMap;

fn line_calibrate(line: &str) -> u32 {
        let digits = line.chars().filter(|c| c.is_digit(10));
        // take the first digit, multiply by 10 and add the last digit.
        let outcome = digits.clone().take(1).next().unwrap().to_digit(10).unwrap() * 10
            + digits.clone().last().unwrap().to_digit(10).unwrap();
            outcome
}

fn line_calibrate2(line: &str) -> u32 {
    let search = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let search_numbers = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let mut index = 0;
    let mut ranks: HashMap<usize, u32> = HashMap::new();
    while index < line.len() {
        let str_part = line.split_at(index).1;
        for (w_index, word) in search.iter().enumerate() {
            if str_part.starts_with(*word) {
                // get the corresponding number
                ranks.insert(index, (w_index as u32) + 1);
            }
        }

        for (w_index, word) in search_numbers.iter().enumerate() {
            if str_part.starts_with(*word) {
                // get the corresponding number
                ranks.insert(index, w_index as u32);
            }
        }
        index += 1;
    }

    // get the value with the lowest index
    let lowest = ranks.iter().min_by_key(|&(i, _)| i).unwrap().1;
    // get the value with the highest index
    let highest = ranks.iter().max_by_key(|&(i, _)| i).unwrap().1;

    // append the first digit to the last digit
    let outcome = lowest * 10 + highest;

    outcome
}

pub fn solve(input: &str) -> u32 {
    input.lines().map(|line| line_calibrate(line)).sum()
}

pub fn solve2(input: &str) -> u32 {
    input.lines().map(|line| line_calibrate2(line)).sum()
}

fn main() {
    let output = solve(include_str!("../../input/day01.txt"));
    println!("Part 1: {}", output);

    let output = solve2(include_str!("../../input/day01.txt"));
    println!("Part 2: {}", output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        let expected = 142;
        assert_eq!(expected, solve(input));
    }

    #[test]
    fn test_line_calibrate_1() {
        let input = "1abc2";
        let expected = 12;
        assert_eq!(expected, line_calibrate(input));
    }

    #[test]
    fn test_line_calibrate_2() {
        let input = "xtwone3four";
        let expected = 24;
        assert_eq!(expected, line_calibrate2(input));
    }

    #[test]
    fn test_part2() {
        let input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;
        let expected = 281;

        assert_eq!(expected, solve2(input));
    }
}
