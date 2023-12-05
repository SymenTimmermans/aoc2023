// The type reveal contains numbers of red green and blue cubes (R, G, B)
type Reveal = (u32, u32, u32);

#[derive(Debug, PartialEq)]
pub struct Game {
    nr: u32,
    reveals: Vec<Reveal>,
}

impl Game {
    pub fn is_solvable(&self, bag: Reveal) -> bool {
        self
            .reveals
            .iter()
            .all(|reveal| self.is_solvable_reveal(reveal, bag))
    }

    pub fn is_solvable_reveal(&self, reveal: &Reveal, bag: Reveal) -> bool {
        // check if the bag contains enough cubes
        !(bag.0 < reveal.0 || bag.1 < reveal.1 || bag.2 < reveal.2)
    }

    pub fn power(&self) -> u32 {
        let min_bag = self.get_min_bag();
        min_bag.0 * min_bag.1 * min_bag.2
    }

    pub fn get_min_bag(&self) -> Reveal {
        let mut min_bag = (0, 0, 0);
        for reveal in &self.reveals {
            min_bag.0 = min_bag.0.max(reveal.0);
            min_bag.1 = min_bag.1.max(reveal.1);
            min_bag.2 = min_bag.2.max(reveal.2);
        }
        min_bag
    }
}

pub fn parse_reveals(line: &str) -> Vec<Reveal> {
    let mut reveals: Vec<Reveal> = Vec::new();
    for part in line.split(";") {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for (_, color) in part.split(",").enumerate() {
            let color = color.trim();
            // split the string into two parts by a space
            let (number, color) = color.split_at(color.find(" ").unwrap());
            let number = number.trim().parse::<u32>().unwrap();
            let color = color.trim();
            if color == "red" {
                r = number;
            } else if color == "green" {
                g = number;
            } else if color == "blue" {
                b = number;
            }
        }
        reveals.push((r, g, b));
    }
    reveals
}

/// Parse a line into a game
pub fn parse_game(line: &str) -> Game {
    let mut reveals: Vec<Reveal> = Vec::new();
    let mut nr = 0;
    for (i, part) in line.split(":").enumerate() {
        if i == 0 {
            // parse the game number
            // split at space, take the second value
            nr = part.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        } else {
            reveals = parse_reveals(part);
        }
    }
    Game { nr, reveals }
}

/// Parse the input into a vector of games
pub fn get_games(input: &str) -> Vec<Game> {
    input.lines().map(|line| parse_game(line.trim())).collect()
}

/// Return the solution for part 1 of the game
pub fn solve(input: &str, bag: Reveal) -> u32 {
    get_games(input)
        .iter()
        .filter(|game| game.is_solvable(bag))
        .map(|game| game.nr)
        .sum()
}

pub fn solve2(input: &str) -> u32 {
    get_games(input).iter().map(|game| game.power()).sum()
}

pub fn main() {
    let output = solve(include_str!("../../input/day02.txt"), (12, 13, 14));
    println!("Part 1: {}", output);

    let output = solve2(include_str!("../../input/day02.txt"));
    println!("Part 2: {}", output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected = 8;
        assert_eq!(expected, solve(input, (12, 13, 14)));
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game {
            nr: 1,
            // R, G, B
            reveals: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)],
        };
        assert_eq!(expected, parse_game(input));
    }

    #[test]
    fn test_part2() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let expected = 2286;
        assert_eq!(expected, solve2(input));
    }
}
