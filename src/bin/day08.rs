use std::collections::HashMap;

const MAX_STEPS: usize = usize::MAX;

/// A node is a name
type Node = String;

/// An instruction can be left or right
#[derive(Debug, PartialEq)]
enum Instruction {
    Left,
    Right,
}

/// A route is a series of instructions to go left or right
type Route = Vec<Instruction>;

fn parse_route(input: &str) -> Route {
    let mut route = Vec::new();

    for c in input.chars() {
        match c {
            'L' => route.push(Instruction::Left),
            'R' => route.push(Instruction::Right),
            _ => panic!("Invalid instruction"),
        }
    }

    route
}

/// There's the definition of a Map, which is a series of paths
/// And a route to take
struct Map {
    route: Route,
    paths: HashMap<Node, (Node, Node)>,
}

/// A map can be created from a string
impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut paths = HashMap::new();
        let mut start_node: Option<Node> = None;

        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        let route = parse_route(first_line);

        // iterate over the rest of the lines
        while let Some(line) = lines.next() {
            // skip empty lines
            if line.is_empty() {
                continue;
            }

            // split the line into two parts
            let mut parts = line.split(" = ");
            let from = parts.next().unwrap().to_owned();

            // if start_node is None, set it to from
            if start_node.is_none() {
                start_node = Some(from.to_owned());
            }

            let to = parts.next().unwrap();

            // split the to part into two nodes
            let mut nodes = to.trim_matches(|c| c == '(' || c == ')').split(", ");

            let left = nodes.next().unwrap().to_owned();
            let right = nodes.next().unwrap().to_owned();

            // add the path to the list of paths
            paths.insert(from, (left, right));
        }

        Map { route, paths }
    }
}

impl Map {
    /// Get the steps to take
    fn get_steps(&self) -> usize {
        let mut steps = 1;
        let mut current = "AAA".to_owned();

        while (current != "ZZZ") && (steps < MAX_STEPS) {
            for instruction in &self.route {
                let (left, right) = self.paths.get(&current).unwrap();

                match instruction {
                    Instruction::Left => {
                        current = left.to_owned();
                    }
                    Instruction::Right => {
                        current = right.to_owned();
                    }
                }

                // if current is ZZZ, we're done
                if current == "ZZZ" {
                    break;
                }

                steps += 1;
            }
        }
        steps
    }

    /// An optimized solution that takes into account that we're dealing with a
    /// tree structure where we can take multiple steps at once. I'm not sure
    /// what the best way to optimize is. A few options:
    /// 1. figure out looping paths, and eliminate them.
    ///    (This is not an option since if we encounter looping paths, starting
    ///    from a node that ends with 'A', we should have taken that path any-
    ///    way.)
    /// 2. Backtrack in reverse.
    ///    Because we know the number of 'start' nodes that end with 'A', we
    ///    can assume that for the puzzle to be 'solvable', that there must be
    ///    at least as many nodes that end with 'Z'.
    ///    The solution will be any of the variants with N nodes ending in 'Z'.
    ///    This way, we can disregard the path entirely, since we're only in-
    ///    terested in figuring out if we could have gotten to this situation
    ///    by continuously taking the same path.
    /// 3. Go over the starting nodes one by one, tracking every time, and
    ///    after how many steps a node ending with 'Z' is found. After doing
    ///    this for every starting node, we only have to find the set of common
    ///    steps that allow the movement from all starting nodes to reach an
    ///    end node with the same number of movements.
    ///
    /// Additionaly, what might help is to keep a cache of how many steps it
    /// takes to get from a node to an end node. But this highly depends on the
    /// movement stack, so that may not be feasible.
    ///
    ///
    pub fn get_better_steps(&self) -> usize {
        let current_nodes: Vec<Node> = self
            .paths
            .keys()
            .filter_map(|x| {
                if x.ends_with('A') {
                    Some(x.to_owned())
                } else {
                    None
                }
            })
            .collect();

        // We figured out that all the paths that lead across end nodes, are
        // looping paths. If we find the loop sizes of every path across a
        // start and end node, we can find the least common multiple, and that
        // values should be the number of steps that will take us from all
        // start nodes to all end nodes.
        let mut loop_sizes = vec![];

        for node in current_nodes.iter() {
            let mut start_node = node.clone();
            let mut integrated_steps = vec![];

            // considering that we start somewhere and that we will see that
            // every path will eventually be a loop of a certain length, we
            // need to find 3 integrated steps to determine the loop length and
            // start offset (if any)

            let mut i = 0;
            while integrated_steps.len() < 3 {
                let steps = self.find_end_node_steps(&mut start_node);
                integrated_steps.extend(steps.iter().map(|x| x + (i * self.route.len())));

                i += 1;
            }

            // for the integrated steps, print the difference between every
            // successive node. We can do this by taking chunks of 2 and taking
            // the difference between them.
            let mut differences = vec![];
            for chunk in integrated_steps.windows(2) {
                differences.push(chunk[1] - chunk[0]);
            }

            // we have discovered that the loops are the same, no start offsets
            // are found.
            // we can therefore use the last value in the differences array
            loop_sizes.push(differences.last().unwrap().to_owned());
        }

        // return the least common multiple of the loop sizes
        lcm(loop_sizes.as_slice()) as usize
    }

    pub fn find_end_node_steps(&self, start_node: &mut Node) -> Vec<usize> {
        let mut end_steps = vec![];
        let mut steps: usize = 1;

        // iterate over the instructions
        for instruction in &self.route {
            let (left, right) = self.paths.get(start_node).unwrap();
            match instruction {
                Instruction::Left => {
                    // modify current to be the left node
                    *start_node = left.to_owned();
                }
                Instruction::Right => {
                    // modify current to be the right node
                    *start_node = right.to_owned();
                }
            }
            // if current is ZZZ, we've found an end node
            if start_node.ends_with('Z') {
                end_steps.push(steps);
            }
            steps += 1;
        }

        end_steps
    }
}

/// Least common multiple vec of numbers
fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn solve(input: &str) -> usize {
    let map = Map::from(input);
    map.get_steps()
}

pub fn solve2(input: &str) -> usize {
    let map = Map::from(input);
    map.get_better_steps()
}

pub fn main() {
    let input = include_str!("../../input/day08.txt");
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
        assert_eq!(
            solve(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }

    #[test]
    fn test_read_map() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let map = Map::from(input);

        assert_eq!(
            map.route,
            vec![Instruction::Left, Instruction::Left, Instruction::Right]
        );
        assert_eq!(map.paths.len(), 3);
        assert_eq!(
            map.paths.get("AAA"),
            Some(&("BBB".to_owned(), "BBB".to_owned()))
        );
        assert_eq!(
            map.paths.get("BBB"),
            Some(&("AAA".to_owned(), "ZZZ".to_owned()))
        );
        assert_eq!(
            map.paths.get("ZZZ"),
            Some(&("ZZZ".to_owned(), "ZZZ".to_owned()))
        );
    }

    #[test]
    fn test_parse_route() {
        assert_eq!(
            parse_route("LLR"),
            vec![Instruction::Left, Instruction::Left, Instruction::Right]
        );
    }
}
