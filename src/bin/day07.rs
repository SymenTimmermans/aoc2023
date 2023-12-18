/// Since a Card can have a value of 2-14, we can use a u8 to represent it.
/// And we can use basically use a hex representation for the value, to support
/// values over 9.
type Card = u8;

/// Looking a the test data, the bid can fit within a u32 easily.
type Bid = u32;

/// As we only see 1000 hands in the test data, we can use a u32 to represent
/// the rank.
type Rank = u32;

/// Since we use hex values for the cards, our hands are always 5 hex characters
/// long, so it would easily fit into a u32. (5x4 = 20 bits, u32 is 32 bits)
type Hand = u32;

/// When valuing a hand, we can simply add a character in front of the hand
/// representation, since there are only 7 kinds of hands. This means that
/// the value of a hand also fits within a u32.
type HandValue = u32;

type HandType = u32;

fn char_to_card(c: char) -> Card {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '*' => 1,
        _ => c.to_digit(10).unwrap() as u8,
    }
}

fn hand_type(hand: Hand) -> HandType {
    // get the hex representation of the hand
    let hand_hex = format!("{:x}", hand);

    // make an array of the number of times each card appears
    let mut card_counts = [0; 15];
    for c in hand_hex.chars() {
        card_counts[c.to_digit(16).unwrap() as usize] += 1;
    }

    let nr_jokers = card_counts[1];

    // if there are any 5s in the array, we have five of a kind
    if card_counts.contains(&5) {
        return 7;
    }

    // if there are any 4s in the array, we have four of a kind
    if card_counts.contains(&4) {
        // if the hand contains a 1, we have a five of a kind instead
        if nr_jokers > 0 {
            return 7;
        }
        return 6;
    }

    // if there are a 3 and a 2 in the array, we have a full house
    if card_counts.contains(&3) && card_counts.contains(&2) {
        // if the hand contains a 1, we have a five of a kind instead
        // since this can only occur as 111xx and 11xxx
        if nr_jokers > 0 {
            return 7;
        }
        return 5;
    }

    // if there are any 3s in the array, we have three of a kind
    if card_counts.contains(&3) {
        // if the hand contains a 1, we have a four of a kind instead 
        // since this can mean xxxy1 or 111xy
        if nr_jokers > 0 {
            return 6;
        }
        return 4;
    }

    // if there are two 2s in the array, we have two pair
    if card_counts.iter().filter(|&&x| x == 2).count() == 2 {
        // if there are two jokers, we have a four of a kind instead
        // since this can only occur as 11xxy
        if nr_jokers == 2 {
            return 6;
        }
        // if we have one joker, we have a full house instead
        // since this can only occur as 1xxyy
        if nr_jokers == 1 {
            return 5;
        }
        return 3;
    }

    // if there is one 2 in the array, we have one pair
    if card_counts.contains(&2) {
        // if there are any jokers, we have a three of a kind instead
        // since this can only occur as 1xxyz or 11xyz
        if nr_jokers > 0 {
            return 4;
        }
        return 2;
    }

    // if we have high card, and we have a joker, we have a pair instead
    if nr_jokers > 0 {
        return 2;
    }

    // return high card
    return 1;
}


fn hand_value(hand: Hand) -> HandValue {
    // get the type of the hand
    let hand_type = hand_type(hand);
    // return the value
    (hand_type << 20) + hand 
}

fn parse_hand(input: &str) -> Hand {
    input.chars().map(char_to_card).enumerate().map(|(i, c)| {
        (c as u32) << (4 * (4 - i))
    }).sum::<u32>()
}

fn parse_input_line(input: &str) -> (Hand, Bid) {
    // take the input and split on a space
    let mut parts = input.split_whitespace();
    // take the first 5 chars of the first part, parse it into a hand
    let hand = parse_hand(&parts.next().unwrap()[..5]);
    // take the second part, parse it into a bid
    let bid = parts.next().unwrap().parse().unwrap();
    // return the tuple
    (hand, bid)
}

fn parse_input(input: &str) -> Vec<(Hand, Bid)> {
    input.lines().map(parse_input_line).collect()
}

fn rank(set: Vec<(Hand, Bid)>) -> Vec<(Rank, Bid)> {
    // sort the set by hand value
    let mut sorted_set = set.clone();
    sorted_set.sort_by(|a, b| hand_value(a.0).cmp(&hand_value(b.0)));

    // create a vector of ranks
    let mut ranks = vec![];
    // create a counter
    // loop through the sorted set
    for (i, (_, bid)) in sorted_set.iter().enumerate() {
        // add the rank to the vector
        ranks.push((i as u32 + 1, *bid));
    }

    ranks
}

pub fn solve(input: &str) -> u32 {
    let hands = parse_input(input);
    let ranks = rank(hands);
    // iterate over the hands and multiply the rank by the bid
    ranks.iter().map(|(rank, bid)| rank * bid).sum()
}

pub fn solve2(input: &str) -> u32 {
    // replace all 'J' with '*' in the input
    let input = input.replace("J", "*");
    return solve(&input);
}


pub fn main() {
    let input = include_str!("../../input/day07.txt");
    println!("Part 1: {}", solve(input));    
    let input = include_str!("../../input/day07.txt");
    println!("Part 2: {}", solve2(input));    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_card() {
        assert_eq!(char_to_card('A'), 14);
        assert_eq!(char_to_card('K'), 13);
        assert_eq!(char_to_card('Q'), 12);
        assert_eq!(char_to_card('J'), 11);
        assert_eq!(char_to_card('T'), 10);
        assert_eq!(char_to_card('9'), 9);
        assert_eq!(char_to_card('8'), 8);
        assert_eq!(char_to_card('7'), 7);
        assert_eq!(char_to_card('6'), 6);
        assert_eq!(char_to_card('5'), 5);
        assert_eq!(char_to_card('4'), 4);
        assert_eq!(char_to_card('3'), 3);
        assert_eq!(char_to_card('2'), 2);
        assert_eq!(char_to_card('*'), 1);
    }

    #[test]
    fn test_example() {
        let input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";

        let outcome = solve(input);
        assert_eq!(outcome, 6440, "part 1");

        let outcome = solve2(input);
        assert_eq!(outcome, 5905, "part 2");
    }
}