use std::{collections::HashMap, convert::identity, str::FromStr};

/// A position on the schematic
type Position = (usize, usize);

/// A schematic of a machine
struct Schematic {
    numbers: HashMap<Position, u32>,
    symbols: HashMap<Position, char>,
    width: i32,
    height: i32,
}

impl Schematic {
    /// Get all of the part numbers
    pub fn part_numbers(&self) -> Vec<u32> {
        let mut part_numbers = Vec::new();
        for (position, number) in &self.numbers {
            if self.is_adjacent(*position, number.to_string().len()) {
                part_numbers.push(*number);
            }
        }
        part_numbers
    }

    // returns true if the position is adjacent to a symbol
    fn is_adjacent(&self, position: Position, length: usize) -> bool {
        for offset in 0..=(length - 1) {
            let (x, y) = position;
            if self
                .get_neighbours((x + offset, y))
                .iter()
                .any(|neighbour| self.symbols.contains_key(neighbour))
            {
                return true;
            }
        }
        false
    }

    /// Returns true if the number is adjacent to the position
    fn nr_is_adjacent_to(
        &self,
        nr_position: Position,
        nr_length: usize,
        search_position: Position,
    ) -> bool {
        for offset in 0..=(nr_length - 1) {
            let position = (nr_position.0 + offset, nr_position.1);
            if self.get_neighbours(position).contains(&search_position) {
                return true;
            }
        }
        false
    }

    /// Get all of the 8 neighbours of a position
    fn get_neighbours(&self, position: Position) -> Vec<Position> {
        let mut neigh = Vec::new();
        let x = position.0 as i32;
        let y = position.1 as i32;
        for p in [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ] {
            if p.0 < 0 || p.1 < 0 {
                continue;
            }
            if p.0 > self.width || p.1 > self.height {
                continue;
            }

            neigh.push((p.0 as usize, p.1 as usize));
        }
        neigh
    }

    /// Get all of the gear ratios for the engine
    fn gear_ratios(&self) -> Vec<u32> {
        self.symbols
            .iter()
            .filter(|(_, symbol)| **symbol == '*')
            .map(|(position, _)| self.get_gear_ratio(*position))
            .filter_map(identity)
            .collect()
    }

    /// Get the gear ratio for a certain gear
    /// (By position)
    fn get_gear_ratio(&self, gear_pos: Position) -> Option<u32> {
        // find two numbers that are both adjacent to the position
        // first, find one number that is adjacent to the position
        for (nr_pos, nr) in &self.numbers {
            if self.nr_is_adjacent_to(*nr_pos, nr.to_string().len(), gear_pos) {
                // find another number that is adjacent to the position
                for (nr_pos2, nr2) in &self.numbers {
                    // if number_position is the same as number_position2, skip
                    if nr_pos == nr_pos2 {
                        continue;
                    }

                    if self.nr_is_adjacent_to(*nr_pos2, nr2.to_string().len(), gear_pos) {
                        return Some(nr * nr2);
                    }
                }
            }
        }
        None
    }
}

impl FromStr for Schematic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = HashMap::new();
        let mut symbols = HashMap::new();
        let height = s.lines().count() as i32;
        // take the fist line and count the number of characters
        let width = s.lines().next().unwrap().chars().count() as i32;
        for (y, line) in s.lines().enumerate() {
            let mut number: String = "".into();
            for (x, c) in line.chars().enumerate() {
                if c.is_digit(10) {
                    number.push(c);
                } else {
                    if !number.is_empty() {
                        numbers.insert((x - number.len(), y), number.parse().unwrap());
                        number = "".into();
                    }
                    // if number is not a period
                    if c != '.' {
                        symbols.insert((x, y), c);
                    }
                }
            }
            if !number.is_empty() {
                numbers.insert((line.len() - number.len(), y), number.parse().unwrap());
            }
        }
        Ok(Schematic {
            numbers,
            symbols,
            width,
            height,
        })
    }
}

/// Get the sum of all of the part numbers
pub fn solve(input: &str) -> u32 {
    let schem = Schematic::from_str(input).unwrap();
    let part_numbers = schem.part_numbers();
    part_numbers.iter().sum()
}

/// Get the sum of all of the gear ratios
pub fn solve2(input: &str) -> u32 {
    let schem = Schematic::from_str(input).unwrap();
    let gear_ratios = schem.gear_ratios();
    gear_ratios.iter().sum()
}

pub fn main() {
    let output = solve(include_str!("../../input/day03.txt"));
    println!("Part 1: {}", output);

    let output = solve2(include_str!("../../input/day03.txt"));
    println!("Part 2: {}", output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let expected = 4361;
        assert_eq!(expected, solve(input));
    }

    #[test]
    fn test_load_schematic() {
        let input = r#"467..114..
...*......
..35..633."#;

        let schematic = Schematic::from_str(input).unwrap();

        println!("{:?}", schematic.numbers);

        assert_eq!(schematic.numbers.len(), 4);

        assert_eq!(schematic.numbers.get(&(0, 0)), Some(&467));
        assert_eq!(schematic.numbers.get(&(5, 0)), Some(&114));
        assert_eq!(schematic.numbers.get(&(2, 2)), Some(&35));
        assert_eq!(schematic.numbers.get(&(6, 2)), Some(&633));

        assert_eq!(schematic.symbols.get(&(3, 1)), Some(&'*'));
    }

    #[test]
    fn test_numbers_connected() {
        let input = r#"467..114..
...*......
..35..633."#;

        let schematic = Schematic::from_str(input).unwrap();

        let part_numbers = schematic.part_numbers();

        assert_eq!(part_numbers.len(), 2);

        // assert that part_numbers contains 467
        assert!(part_numbers.contains(&467));
        // assert that part_numbers contains 35
        assert!(part_numbers.contains(&35));
    }

    #[test]
    fn test_neighbours() {
        let input = r#"467..114..
...*......
..35..633."#;

        let schematic = Schematic::from_str(input).unwrap();

        let neighbours = schematic.get_neighbours((1, 1));

        assert_eq!(neighbours.len(), 8);

        // neighbours should contain (0, 0)
        assert!(neighbours.contains(&(0, 0)));
        // neighbours should contain (1, 0)
        assert!(neighbours.contains(&(1, 0)));
        // neighbours should contain (2, 0)
        assert!(neighbours.contains(&(2, 0)));
        // neighbours should contain (0, 1)
        assert!(neighbours.contains(&(0, 1)));
        // neighbours should contain (2, 1)
        assert!(neighbours.contains(&(2, 1)));
        // neighbours should contain (0, 2)
        assert!(neighbours.contains(&(0, 2)));
        // neighbours should contain (1, 2)
        assert!(neighbours.contains(&(1, 2)));
        // neighbours should contain (2, 2)
        assert!(neighbours.contains(&(2, 2)));
    }

    #[test]
    fn test_part_2() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        let expected = 467835;

        let outcome = solve2(input);

        assert_eq!(expected, outcome);
    }
}
