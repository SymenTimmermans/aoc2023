use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

/// Day 11: Cosmic Expansion
///
/// We're given a map with Galaxies which should be read into a data structure.
/// It should also be "expanded" before we work on it.
///
type Pos = (i32, i32);

#[derive(Debug, PartialEq, Eq, Hash)]
struct Galaxy(Pos);

struct Map {
    galaxies: Vec<Galaxy>,
}

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Galaxy((x as i32, y as i32)));
                }
            }
        }

        Ok(Map { galaxies })
    }
}

impl Map {
    pub fn expand_once(&mut self) {
        // every empty row becomes twice it's size
        self.expand_times(2);
    }

    pub fn expand_times(&mut self, times: i32) {
        // each row and column that have no galaxies will double up.
        // we need to recalculate the locations of the galaxies
        // based on the columns and rows that have doubled up.
        // we can do this in two steps, columns, then rows.

        // for columns and rows, we can create hashmaps that return
        // the new column for each given column, and the new row
        // for each given row.

        // First, create a hashset that has all the columns.
        let columns: HashSet<i32> = self.galaxies.iter().map(|g| g.0 .0).collect();
        // Now we can create a hashtable that holds the new columns for each
        // old column. For each column we are going to count how many columns
        // before it that don't have galaxies (are not in the hashset).
        let new_columns: HashMap<i32, i32> = columns
            .iter()
            .map(|x| {
                let galaxies_before = columns.iter().filter(|&c| c < x).count() as i32;
                let empty_before = *x - galaxies_before;
                let new_x = *x + (empty_before * (times - 1));
                (*x, new_x)
            })
            .collect();

        // Now we can do the same for the rows.
        let rows: HashSet<i32> = self.galaxies.iter().map(|g| g.0 .1).collect();
        // Now we can create a hashtable that holds the new rows for each
        // old row. For each row we are going to count how many rows
        // before it that don't have galaxies (are not in the hashset).
        let new_rows: HashMap<i32, i32> = rows
            .iter()
            .map(|y| {
                let galaxies_before = rows.iter().filter(|&r| r < y).count() as i32;
                let empty_before = *y - galaxies_before;
                let new_y = *y + (empty_before * (times - 1));
                (*y, new_y)
            })
            .collect();

        // now we can iterate over the galaxy positions and update them
        for galaxy in &mut self.galaxies {
            let (x, y) = galaxy.0;
            galaxy.0 = (new_columns[&x], new_rows[&y]);
        }
    }

    /// Find the sum of all of the shortest paths between each pair of
    /// galaxies.
    pub fn sum_shortest_paths(&self) -> usize {
        self.galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                let galaxy1 = &pair[0];
                let galaxy2 = &pair[1];
                let (x1, y1) = galaxy1.0;
                let (x2, y2) = galaxy2.0;
                let dx = x1 - x2;
                let dy = y1 - y2;
                (dx.abs() + dy.abs()) as usize
            })
            .sum()
    }
}

fn solve(input: &str) -> usize {
    let mut map = input.parse::<Map>().unwrap();
    map.expand_once();
    map.sum_shortest_paths()
}

fn solve2(input: &str) -> usize {
    let mut map = input.parse::<Map>().unwrap();
    map.expand_times(1_000_000);
    map.sum_shortest_paths()
}

pub fn main() {
    let input = include_str!("../../input/day11.txt");

    let output = solve(input);
    println!("Part 1: {}", output);

    let output = solve2(input);
    println!("Part 2: {}", output);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_map() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let map = input.parse::<Map>().unwrap();
        assert_eq!(map.galaxies.len(), 9);
    }

    #[test]
    fn test_expand_map() {
        let input = r#".#."#;
        let mut map = input.parse::<Map>().unwrap();
        // before expansion, there should be one galaxy at (1, 0)
        assert_eq!(map.galaxies.len(), 1);
        assert_eq!(map.galaxies[0].0, (1, 0));

        map.expand_once();
        // after expansion, there should be one galaxy at (2, 0)
        assert_eq!(map.galaxies.len(), 1);
        assert_eq!(map.galaxies[0].0, (2, 0));
    }

    #[test]
    fn test_expand_bigger_map() {
        let input = r#"...#..."#;
        let mut map = input.parse::<Map>().unwrap();
        // before expansion, there should be one galaxy at (3, 0)
        assert_eq!(map.galaxies.len(), 1);
        assert_eq!(map.galaxies[0].0, (3, 0));

        map.expand_once();
        // after expansion, there should be one galaxy at (6, 0)
        assert_eq!(map.galaxies.len(), 1);
        assert_eq!(map.galaxies[0].0, (6, 0));
    }

    #[test]
    fn test_expand_multiple_galaxies() {
        let input = r#"...#.#.."#;
        let mut map = input.parse::<Map>().unwrap();
        // before expansion, there should be one galaxy at (3, 0)
        // and one at (5, 0)
        // ...#.#..
        // 01234567
        assert_eq!(map.galaxies.len(), 2);
        assert_eq!(map.galaxies[0].0, (3, 0));
        assert_eq!(map.galaxies[1].0, (5, 0));

        map.expand_once();
        // after expansion, there should be one galaxy at (6, 0)
        // and one at (9, 0)
        // ......#..#....
        // 01234567890123
        assert_eq!(map.galaxies.len(), 2);
        assert_eq!(map.galaxies[0].0, (6, 0));
        assert_eq!(map.galaxies[1].0, (9, 0));
    }

    #[test]
    fn test_expand_vertical_multiple_galaxies() {
        let input = r#".
.
.
#
.
#
.
."#;
        let mut map = input.parse::<Map>().unwrap();
        // before expansion, there should be one galaxy at (0, 3)
        // and one at (0, 5)
        assert_eq!(map.galaxies.len(), 2);
        assert_eq!(map.galaxies[0].0, (0, 3));
        assert_eq!(map.galaxies[1].0, (0, 5));
        map.expand_once();
        // after expansion, there should be one galaxy at (0, 6)
        // and one at (0, 9)
        assert_eq!(map.galaxies.len(), 2);
        assert_eq!(map.galaxies[0].0, (0, 6));
        assert_eq!(map.galaxies[1].0, (0, 9));
    }

    #[test]
    fn test_example_map_expands_correctly() {
        let input_original = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let input_expanded = r#"....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."#;

        let mut orig_map = input_original.parse::<Map>().unwrap();
        let expanded_map = input_expanded.parse::<Map>().unwrap();
        // expand the original map
        orig_map.expand_once();

        // both maps should have 9 galaxies
        assert_eq!(orig_map.galaxies.len(), 9);
        assert_eq!(expanded_map.galaxies.len(), 9);

        // both maps should have the same galaxies
        for galaxy in &orig_map.galaxies {
            assert!(expanded_map.galaxies.contains(galaxy));
        }
    }

    #[test]
    fn test_shortest_paths() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let mut map = input.parse::<Map>().unwrap();
        map.expand_once();
        assert_eq!(map.sum_shortest_paths(), 374);
    }

    #[test]
    fn test_shortest_paths_part2() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let mut map = input.parse::<Map>().unwrap();
        map.expand_times(10);
        assert_eq!(map.sum_shortest_paths(), 1030);

        let mut map = input.parse::<Map>().unwrap();
        map.expand_times(100);
        assert_eq!(map.sum_shortest_paths(), 8410);
    }
}
