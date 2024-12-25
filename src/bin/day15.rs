use std::{fmt::Display, str::FromStr};

use aoc2024::aoc::{self, grid::Grid};

mod parse {
    use aoc2024::aoc::grid::Direction;

    pub fn parse_moves(s: &str) -> Vec<Direction> {
        s.chars().filter_map(parse_direction).collect()
    }

    fn parse_direction(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }
}

type Map = Grid<Vec<Tile>>;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Robot,
    Obstacle,
    Wall,
}

impl Tile {}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "@" => Ok(Self::Robot),
            "O" => Ok(Self::Obstacle),
            "#" => Ok(Self::Wall),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => '.',
                Tile::Robot => '@',
                Tile::Obstacle => 'O',
                Tile::Wall => '#',
            }
        )
    }
}

mod part1 {
    use std::collections::VecDeque;

    use aoc::grid::Point;
    use parse::parse_moves;

    use super::*;

    fn score(map: &Map) -> usize {
        map.iter()
            .map(|cell| match *cell.value() {
                Tile::Obstacle => 100 * cell.y + cell.x,
                _ => 0,
            })
            .sum()
    }

    fn find_robot_point(map: &Map) -> Point {
        map.iter()
            .find(|cell| matches!(*cell.value(), Tile::Robot))
            .expect("Could not find the robot")
            .point()
    }

    pub fn calculate(input: &str) -> usize {
        let (map, moves) = input.split_once("\n\n").unwrap();
        let moves = parse_moves(moves);
        let map = map.trim().parse::<Map>().unwrap();
        let mut robot_point = find_robot_point(&map);

        'moves: for direction in moves {
            // println!(
            //     "Step {}/{} ({})\n\n{}\n",
            //     i + 1,
            //     total_moves,
            //     direction,
            //     map
            // );
            let mut queue = VecDeque::new();
            queue.push_back(robot_point);
            let mut cell = map.cell_at_point(&robot_point).unwrap();
            while let Some(c) = cell.go(&direction) {
                match *c.value() {
                    Tile::Empty => {
                        queue.push_back(c.point());
                        break;
                    }
                    Tile::Obstacle => {
                        queue.push_back(c.point());
                        cell = c;
                    }
                    Tile::Wall => continue 'moves,
                    Tile::Robot => unreachable!("Should not find robot"),
                }
            }

            let mut last = None;

            while let Some(point) = queue.pop_front() {
                let mut cell = map.cell_at_point(&point).unwrap();
                let this = *cell.value();
                *cell.value_mut() = last.unwrap_or(Tile::Empty);
                if matches!(*cell.value(), Tile::Robot) {
                    robot_point = point;
                }
                last = Some(this);
            }
        }

        #[cfg(test)]
        println!("\n{}", map);

        score(&map)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day15.txt");
            assert_eq!(calculate(&input), 10092);
        }

        #[test]
        fn test_example_small() {
            let input = aoc::example::example_string("day15_small.txt");
            assert_eq!(calculate(&input), 2028);
        }
    }
}
/*
mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day15.txt");
            assert_eq!(calculate(&input), 0);
        }
    }
}
*/
fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    // println!("Part 2: {}", part2::calculate(&input));
}
