use std::str::FromStr;

/// Day 10
///
/// The input describes a loop of pipes. We have to find the longest distance
/// from the start of the loop.
///
/// Based on the representation we choose, this can be calculated quite easily.
/// If we manage to read the pipe network into a vec, then we're able to just
/// take half of the length of the vec to get the farthest distance from the
/// start.

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pipe {
    Start,
    EastWest,
    NorthSouth,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    Outside,
    Inside,
}

impl Pipe {
    pub fn connects(&self, dir: Direction) -> bool {
        match self {
            Pipe::Start => true,
            Pipe::Ground => false,
            Pipe::Outside => false,
            Pipe::Inside => false,
            _ => match dir {
                Direction::North => match self {
                    Pipe::NorthSouth => true,
                    Pipe::NorthEast => true,
                    Pipe::NorthWest => true,
                    _ => false,
                },
                Direction::South => match self {
                    Pipe::NorthSouth => true,
                    Pipe::SouthEast => true,
                    Pipe::SouthWest => true,
                    _ => false,
                },
                Direction::East => match self {
                    Pipe::EastWest => true,
                    Pipe::NorthEast => true,
                    Pipe::SouthEast => true,
                    _ => false,
                },
                Direction::West => match self {
                    Pipe::EastWest => true,
                    Pipe::NorthWest => true,
                    Pipe::SouthWest => true,
                    _ => false,
                },
            },
        }
    }

    /// Given a direction, return the direction and position of the next pipe
    pub fn pass_from(&self, dir: Direction, (x, y): Pos) -> (Direction, Pos) {
        match dir {
            Direction::North => match self {
                Pipe::SouthEast => (Direction::East, (x + 1, y)),
                Pipe::SouthWest => (Direction::West, (x - 1, y)),
                Pipe::NorthSouth => (Direction::North, (x, y - 1)),
                _ => panic!("Can't pass {:?}-wards through {:?}", dir, self),
            },
            Direction::South => match self {
                Pipe::NorthEast => (Direction::East, (x + 1, y)),
                Pipe::NorthWest => (Direction::West, (x - 1, y)),
                Pipe::NorthSouth => (Direction::South, (x, y + 1)),
                _ => panic!("Can't pass {:?}-wards through {:?}", dir, self),
            },
            Direction::East => match self {
                Pipe::NorthWest => (Direction::North, (x, y - 1)),
                Pipe::SouthWest => (Direction::South, (x, y + 1)),
                Pipe::EastWest => (Direction::East, (x + 1, y)),
                _ => panic!("Can't pass {:?}-wards through {:?}", dir, self),
            },
            Direction::West => match self {
                Pipe::NorthEast => (Direction::North, (x, y - 1)),
                Pipe::SouthEast => (Direction::South, (x, y + 1)),
                Pipe::EastWest => (Direction::West, (x - 1, y)),
                _ => panic!("Can't pass {:?}-wards through {:?}", dir, self),
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

// impl From<char> for Pipe
impl From<char> for Pipe {
    fn from(c: char) -> Self {
        match c {
            'S' => Pipe::Start,
            '-' => Pipe::EastWest,
            '|' => Pipe::NorthSouth,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'O' => Pipe::Outside,
            'I' => Pipe::Inside,
            _ => panic!("Unknown pipe type: {}", c),
        }
    }
}

// impl Into<char> for Pipe
impl Into<char> for Pipe {
    fn into(self) -> char {
        match self {
            Pipe::Start => 'S',
            Pipe::EastWest => '─',
            Pipe::NorthSouth => '│',
            Pipe::NorthEast => '└',
            Pipe::NorthWest => '┘',
            Pipe::SouthWest => '┐',
            Pipe::SouthEast => '┌',
            Pipe::Ground => '·',
            Pipe::Outside => 'O',
            Pipe::Inside => 'I',
        }
    }
}

type Pos = (usize, usize);

struct Map(Vec<Vec<Pipe>>);

impl FromStr for Map {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().map(|c| Pipe::from(c)).collect())
            .collect();
        Ok(Map(map))
    }
}

// impl display for Map
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.0 {
            for pipe in row {
                write!(f, "{}", Into::<char>::into(*pipe))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    pub fn find_a_start(&self, pos: Pos) -> (Direction, Pos) {
        let mut direction = Direction::South;
        let mut pos = pos;

        // go up and check if there's a pipe that connects down
        let mut moved = false;
        if pos.1 > 0 {
            if let Some(pipe_up) = self.get_pipe(pos.0, pos.1 - 1) {
                if pipe_up.connects(Direction::South) {
                    pos.1 -= 1;
                    moved = true;
                    direction = Direction::North
                }
            }
        }
        // go right and check if there's a pipe that connects left
        if pos.0 < self.width() - 1 {
            if !moved {
                if let Some(pipe_right) = self.get_pipe(pos.0 + 1, pos.1) {
                    if pipe_right.connects(Direction::West) {
                        pos.0 += 1;
                        moved = true;
                        direction = Direction::East;
                    }
                }
            }
        }
        // go down and check if there's a pipe that connects up
        if pos.1 < self.height() - 1 {
            if !moved {
                if let Some(pipe_down) = self.get_pipe(pos.0, pos.1 + 1) {
                    if pipe_down.connects(Direction::North) {
                        pos.1 += 1;
                        moved = true;
                        direction = Direction::South;
                    }
                }
            }
        }
        // go left and check if there's a pipe that connects right
        if pos.0 > 0 {
            if !moved {
                if let Some(pipe_left) = self.get_pipe(pos.0 - 1, pos.1) {
                    if pipe_left.connects(Direction::East) {
                        pos.0 -= 1;
                        moved = true;
                        direction = Direction::West;
                    }
                }
            }
        }

        if !moved {
            panic!("Failed to find a loop");
        }

        (direction, pos)
    }

    /// When we have that 2d grid, we can find the loop that's inside of it.
    /// We need to start from the 'S', and find at least one neighbouring pipe.
    /// Then, keep following pipes until we find an S again.
    /// After that, we should have the length of the loop.
    pub fn loop_length(&self) -> usize {
        let start_pos = self.start();
        let mut moves: usize = 1;
        let (mut direction, mut pos) = self.find_a_start(start_pos);

        // from this point on, we can follow the pipes until we
        // find the start again
        while pos != start_pos {
            // get the pipe we're on
            let pipe = self.get_pipe(pos.0, pos.1).unwrap();

            // get the new position and direction based on the
            // current pipe and direction
            (direction, pos) = pipe.pass_from(direction, pos);
            moves += 1;
        }

        moves
    }

    pub fn loop_positions(&self) -> Vec<Pos> {
        let start_pos = self.start();
        let mut positions = vec![start_pos];
        let (mut direction, mut pos) = self.find_a_start(start_pos);
        // from this point on, we can follow the pipes until we
        // find the start again
        while pos != start_pos {
            positions.push(pos);
            // get the pipe we're on
            let pipe = self.get_pipe(pos.0, pos.1).unwrap();
            // get the new position and direction based on the
            // current pipe and direction
            (direction, pos) = pipe.pass_from(direction, pos);
        }
        positions
    }

    /// get the pipe at the given position
    pub fn get_pipe(&self, x: usize, y: usize) -> Option<Pipe> {
        self.0.get(y).and_then(|row| row.get(x).cloned())
    }

    pub fn set_pipe(&mut self, x: usize, y: usize, pipe: Pipe) {
        self.0[y][x] = pipe;
    }

    /// find the start position
    pub fn start(&self) -> Pos {
        self.0
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == Pipe::Start {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap()
    }

    pub fn nr_inside(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&c| c == Pipe::Inside)
            .count()
    }

    pub fn mark_inside(&mut self) {
        // first we need to change everything that's not part of the loop, into
        // ground
        let loop_positions = self.loop_positions();

        for (y, row) in self.0.iter_mut().enumerate() {
            for (x, pipe) in row.iter_mut().enumerate() {
                if !loop_positions.contains(&(x, y)) {
                    *pipe = Pipe::Ground;
                }
            }
        }
        // We need to iterate over tiles within the bounds of the loop.
        // A stripwise approach could work fine, as long as we keep track
        // whether we're inside, outside, or on the border. And, we should keep
        // track of the direction of outside, since this could help us deter-
        // mine if we end up inside after corners.
        //
        // There's a few possibilities of pipes we can encounter on a line.
        // |...|      -> easy, we go inside and outside again.
        // |..|..|..| -> inside, outside, inside
        // F---7..... -> remain outside
        // L---J..... -> remain outside
        // |..F-7...| -> inside, outside, inside
        // |..L-7...| -> inside, outside
        // FJ...LJ.L7 -> inside, inside
        //
        // Basically, if we keep track of the last corner, we know if we have
        // crossed inside or outside.

        // group the positions of the loop into a hashmap with y as key
        let mut v_pipe_groups = std::collections::HashMap::new();
        for (x, y) in self.loop_positions() {
            v_pipe_groups.entry(y).or_insert(Vec::new()).push(x);
        }

        // loop from the lowest x+1 to the highest x-1
        for (y, x_positions) in v_pipe_groups {
            let min_x = x_positions.iter().min().unwrap() + 0;
            let max_x = x_positions.iter().max().unwrap() + 0;
            // keep track of whether we're inside or outside
            let mut inside = false;
            // Keep track of the last corner and initialize it as ground to
            // indicate it's not a corner
            let mut last_corner = Pipe::Ground;
            for x in min_x..=max_x {
                let mut tile = self.get_pipe(x, y).unwrap();
                if tile == Pipe::Start {
                    tile = self.deduct_pipe(x, y);
                }
                match tile {
                    Pipe::Ground => {
                        if inside {
                            self.set_pipe(x, y, Pipe::Inside);
                        } else {
                            self.set_pipe(x, y, Pipe::Outside);
                        }
                    }
                    Pipe::NorthSouth => inside = !inside,
                    Pipe::NorthEast => last_corner = Pipe::NorthEast,
                    Pipe::NorthWest => {
                        if last_corner == Pipe::SouthEast {
                            inside = !inside;
                        }
                    }
                    Pipe::SouthEast => last_corner = Pipe::SouthEast,
                    Pipe::SouthWest => {
                        if last_corner == Pipe::NorthEast {
                            inside = !inside;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    /// Deduct what pipe is at x, y by looking at the surrounding pipes
    /// Take into account we might be at a border of the map
    pub fn deduct_pipe(&self, x: usize, y: usize) -> Pipe
    {
        // Are we connected above?
        let connected_above = if y > 0 {
            self.get_pipe(x, y - 1).unwrap().connects(Direction::South)
        } else {
            false
        };

        // Are we connected below?
        let connected_below = if y < self.height() - 1 {
            self.get_pipe(x, y + 1).unwrap().connects(Direction::North)
        } else {
            false
        };

        // Are we connected left?
        let connected_left = if x > 0 {
            self.get_pipe(x - 1, y).unwrap().connects(Direction::East)
        } else {
            false
        };

        // Are we connected right?
        let connected_right = if x < self.width() - 1 {
            self.get_pipe(x + 1, y).unwrap().connects(Direction::West)
        } else {
            false
        };

        if connected_above && connected_below {
            Pipe::NorthSouth
        } else if connected_left && connected_right {
            Pipe::EastWest
        } else if connected_above && connected_left {
            Pipe::NorthWest
        } else if connected_above && connected_right {
            Pipe::NorthEast
        } else if connected_below && connected_left {
            Pipe::SouthWest
        } else if connected_below && connected_right {
            Pipe::SouthEast
        } else {
            Pipe::Ground
        }
    }
}

fn solve(input: &str) -> usize {
    let map = Map::from_str(input).expect("Failed to parse map");
    map.loop_length() / 2
}

fn solve2(input: &str) -> usize {
    let mut map = Map::from_str(input).expect("Failed to parse map");
    map.mark_inside();
    println!("{}", map);
    map.nr_inside()
}

pub fn main() {
    let input = include_str!("../../input/day10.txt");
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
        assert_eq!(
            4,
            solve(
                r#".....
.S-7.
.|.|.
.L-J.
....."#
            )
        );
    }

    #[test]
    fn test_start_pos() {
        let map = Map::from_str(
            r#".....
.S-7.
.|.|.
.L-J.
....."#,
        )
        .expect("Failed to parse map");
        assert_eq!((1, 1), map.start());
    }

    #[test]
    fn test_loop_length() {
        let map = Map::from_str(
            r#".....
.S-7.
.|.|.
.L-J.
....."#,
        )
        .expect("Failed to parse map");
        assert_eq!(8, map.loop_length());
    }

    #[test]
    fn test_count_inside() {
        let map = Map::from_str(
            r#"..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."#,
        )
        .expect("Failed to parse map");
        assert_eq!(4, map.nr_inside());
    }

    #[test]
    fn test_solve2() {
        assert_eq!(
            4,
            solve2(
                r#"..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#
            )
        );
    }

    #[test]
    fn test_solve2_start_problem() {
        assert_eq!(
            4,
            solve2(
                r#"..........
.F------7.
.|F----7|.
.||....||.
.S|....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
.........."#
            )
        );
    }

    #[test]
    fn test_solve2b() {
        assert_eq!(
            11,
            solve2(
                r#"..........
.S-7......
.|.L-7....
.|...L--7.
.|..F-7.|.
.|.FJ.|.|.
.|.|..|.|.
.L-J..L-J.
.........."#
            )
        );
    }

    #[test]
    fn test_solve2c() {
        assert_eq!(
            10,
            solve2(
                r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
            )
        );
    }
}
