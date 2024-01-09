use std::str::FromStr;

/// A sequence is a list of numbers
struct Sequence(Vec<i32>);

/// We need to be able to read an aribtrary string of numbers separated by a
/// space character into a sequence.
impl FromStr for Sequence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sequence = Vec::new();
        for number in s.split(" ") {
            sequence.push(number.parse().unwrap());
        }
        Ok(Sequence(sequence))
    }
}

impl Sequence {
    pub fn differences(&self) -> Sequence {
        let mut diffs = Vec::new();
        for i in 1..self.0.len() {
            diffs.push(self.0[i] - self.0[i - 1]);
        }
        Sequence(diffs)
    }

    pub fn is_all_zeroes(&self) -> bool {
        for i in 0..self.0.len() {
            if self.0[i] != 0 {
                return false;
            }
        }
        true
    }

    /// Extrapolate a sequence by
    /// predicting the next number
    pub fn extrapolate(&self) -> i32 {
        if self.is_all_zeroes() {
            return 0;
        }

        let last = self.0.last().unwrap();
        self.differences().extrapolate() + last
    }

    /// Extrapolate a sequence by
    /// predicting the next number in the front! :-)
    pub fn extrapolate_front(&self) -> i32 {
        if self.is_all_zeroes() {
            return 0;
        }

        let first = self.0.first().unwrap();
        let exp = self.differences().extrapolate_front();
        first - exp
    }
}

pub fn solve(input: &str) -> i32 {
    // naive approach, i guess.
    // add all extrapolated values together for each sequence in the input
    let sequences = input
        .lines()
        .map(|line| Sequence::from_str(line).unwrap())
        .collect::<Vec<_>>();
    sequences.iter().map(|seq| seq.extrapolate()).sum()
}

pub fn solve2(input: &str) -> i32 {
    // naive approach, i guess.
    // add all extrapolated values together for each sequence in the input
    let sequences = input
        .lines()
        .map(|line| Sequence::from_str(line).unwrap())
        .collect::<Vec<_>>();
    sequences.iter().map(|seq| seq.extrapolate_front()).sum()
}

pub fn main() {
    let input = include_str!("../../input/day09.txt");
    let output = solve(input);
    println!("Part 1: {}", output);
    let output = solve2(input);
    println!("Part 2: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        let output = solve(input);

        assert_eq!(output, 114);
    }

    #[test]
    fn test_part2() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        let output = solve2(input);

        assert_eq!(output, 2);
    }

    #[test]
    fn test_read_sequence() {
        let input = "1 2 3 4 5";
        let sequence = Sequence::from_str(input).unwrap();
        assert_eq!(sequence.0, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_read_input() {
        let input = "1 2 3 4 5
6 7 8 9 10";
        let sequences = input
            .lines()
            .map(|line| Sequence::from_str(line).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(sequences[0].0, vec![1, 2, 3, 4, 5]);
        assert_eq!(sequences[1].0, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_difference_sequence() {
        let seq = Sequence(vec![0, 3, 6, 9, 12]);
        let diffs = seq.differences();
        assert_eq!(diffs.0, vec![3, 3, 3, 3]);

        let diffs2 = diffs.differences();
        assert_eq!(diffs2.0, vec![0, 0, 0]);
    }

    #[test]
    fn test_is_all_zeroes() {
        let seq = Sequence(vec![0, 0, 0, 0, 0]);
        assert!(seq.is_all_zeroes());
    }

    #[test]
    fn test_extrapolate_zeroes() {
        let seq = Sequence(vec![0, 0, 0, 0]);
        let new = seq.extrapolate();

        assert_eq!(new, 0);
    }

    #[test]
    fn test_extrapolate() {
        let seq = Sequence(vec![0, 3, 6, 9]);
        let new = seq.extrapolate();
        assert_eq!(new, 12);
    }

    #[test]
    fn test_extrapolate_front_zeroes() {
        let seq = Sequence(vec![0, 0, 0, 0]);
        let new = seq.extrapolate_front();
        assert_eq!(new, 0);
    }

    #[test]
    fn test_extrapolate_front_increasing() {
        assert_eq!(Sequence(vec![0, 2, 4, 6]).extrapolate_front(), -2);
        assert_eq!(Sequence(vec![3, 3, 5, 9, 15]).extrapolate_front(), 5);
    }
}
