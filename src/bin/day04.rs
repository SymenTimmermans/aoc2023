use std::str::FromStr;

pub struct Card {
    pub winners: Vec<u32>,
    pub numbers: Vec<u32>,
}

impl Card {
    /// Returns the number of matches between the winners and the numbers.
    pub fn nr_matches(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|number| self.winners.contains(number))
            .count() as u32
    }

    /// Returns the score of the card.
    pub fn score(&self) -> u32 {
        let score = self.nr_matches();

        if score == 0 {
            return 0;
        }

        (2 as u32).pow(score - 1)
    }
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut winners = Vec::new();
        let mut numbers = Vec::new();

        let mut parts = s.split(" | ");
        let mut winners_parts = parts.next().unwrap().split(": ");
        let numbers_parts = parts.next().unwrap().trim().split(" ");

        winners_parts.next();
        for winner in winners_parts.next().unwrap().trim().split(" ") {
            // skip empty winners
            if winner == "" {
                continue;
            }
            winners.push(winner.parse().unwrap());
        }

        for number in numbers_parts {
            // skip empty numbers
            if number == "" {
                continue;
            }
            numbers.push(number.parse().unwrap());
        }

        Ok(Card { winners, numbers })
    }
}

/// Solve part 1 of the puzzle, calculate the score of the scratchcards.
pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .map(|card| card.score())
        .sum()
}

/// Solve part 2 of the puzzle, calculate the number of
/// scratchcards based on the new rules.
pub fn solve2(input: &str) -> u32 {
    let cards = input.lines().map(|line| Card::from_str(line).unwrap());

    let count = cards.clone().count();
    let mut copies = vec![1; count];

    for (i, card) in cards.enumerate() {
        let nr_matches = card.nr_matches();
        let from = i + 1;
        let to = (i + nr_matches as usize + 1).min(count);
        for j in from..to {
            copies[j] += copies[i];
        }
    }

    // return the sum of all the copies
    copies.iter().sum()
}

/// Main function that executes both parts.
pub fn main() {
    let input = include_str!("../../input/day04.txt");

    let output = solve(input);

    println!("Part 1: {}", output);

    let output = solve2(input);

    println!("Part 2: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let output = solve(input);

        assert_eq!(output, 13);
    }

    #[test]
    fn test_solve2() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        let output = solve2(input);

        assert_eq!(output, 30);
    }
}
