#[derive(Debug, PartialEq)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn all_strats(&self) -> Vec<(u64, u64)> {
        (1..(self.time - 1))
            .map(|hold| (hold, (self.time - hold) * hold))
            .collect::<Vec<_>>()
    }

    pub fn winning_strats(&self) -> Vec<u64> {
        self.all_strats()
            .into_iter()
            .filter(|(_, d)| *d > self.distance)
            .map(|(h, _)| h)
            .collect()
    }
}

/// Parse input into a vec of Races
pub fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    // should be two lines only
    assert_eq!(lines.clone().count(), 2);
    // Take the first line, trim the first 5 chars, split on whitespace
    let times = lines.next().unwrap()[5..].split_whitespace();
    // Take the second line, trim the first 9 chars, split on whitespace
    let distances = lines.next().unwrap()[9..].split_whitespace();
    // Zip the two iterators together, map to Race, collect into a vec
    times
        .zip(distances)
        .map(|(t, d)| Race {
            time: t.parse().unwrap(),
            distance: d.parse().unwrap(),
        })
        .collect::<Vec<_>>()
}

pub fn solve(input: &str) -> u64 {
    let races = parse(input);
    races
        .iter()
        .map(|r| r.winning_strats().len() as u64)
        .product()
}

pub fn main() {
    let input = include_str!("../../input/day06.txt");
    println!("Part 1: {}", solve(input));
    let input = include_str!("../../input/day06-2.txt");
    println!("Part 2: {}", solve(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_winning_strats() {
        let r = Race {
            time: 7,
            distance: 9,
        };
        assert_eq!(r.winning_strats(), vec![2, 3, 4, 5]);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(
                "Time:      7  15   30
Distance:   9   40 200"
            ),
            vec![
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                }
            ]
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(
            solve(
                "Time:      7  15   30
Distance:  9  40  200"
            ),
            288
        );
    }

    #[test]
    fn test_solve2() {
        assert_eq!(
            solve(
                "Time:      71530
Distance:  940200"
            ),
            71503
        );
    }
}
