use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use aoc2024::aoc::{
    self,
    grid::{Axis, Direction, Grid, Point},
};

mod parse {
    use aoc2024::aoc::grid::Direction;

    pub fn parse_moves(s: &str) -> Vec<Direction> {
        s.chars().filter_map(parse_direction).collect()
    }

    fn parse_direction(c: char) -> Option<Direction> {
        Some(match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => None?,
        })
    }
}

mod part1 {
    use super::*;

    type Map = Grid<Vec<Tile>>;

    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Empty,
        Robot,
        Obstacle,
        Wall,
    }

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
        let moves = parse::parse_moves(moves);
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

mod part2 {
    use super::*;

    type Map = Grid<Vec<Tile>>;

    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Empty,
        Robot,
        ObstacleLeft,
        ObstacleRight,
        Wall,
    }

    impl FromStr for Tile {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "." => Ok(Self::Empty),
                "@" => Ok(Self::Robot),
                "[" => Ok(Self::ObstacleLeft),
                "]" => Ok(Self::ObstacleRight),
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
                    Tile::ObstacleLeft => '[',
                    Tile::ObstacleRight => ']',
                    Tile::Wall => '#',
                }
            )
        }
    }

    fn widen(input: &str) -> String {
        input
            .chars()
            .map(|c| match c {
                '#' => "##",
                'O' => "[]",
                '.' => "..",
                '@' => "@.",
                '\n' => "\n",
                _ => panic!("Unexpected char '{:?}'", c),
            })
            .collect()
    }

    fn score(map: &Map) -> usize {
        map.iter()
            .map(|cell| match *cell.value() {
                Tile::ObstacleLeft => 100 * cell.y + cell.x,
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

    fn evaluate(input: &str) -> Map {
        let (map, moves) = input.split_once("\n\n").unwrap();
        let moves = parse::parse_moves(moves);
        let map = widen(map.trim()).parse::<Map>().unwrap();
        let mut robot_point = find_robot_point(&map);

        'moves: for direction in moves {
            let mut queue = VecDeque::new();
            let mut to_move = vec![];
            let mut checked = HashSet::new();

            queue.push_back(robot_point);

            while let Some(point) = queue.pop_front() {
                if !checked.insert(point) {
                    continue;
                }
                let cell = point.on(&map).expect("Point in queue must be on map");
                let tile = *cell.value();

                match tile {
                    Tile::Empty => continue,
                    Tile::Robot => {
                        queue.push_back(
                            (point + direction).expect("Robot should not be at edge of map"),
                        );
                    }
                    Tile::ObstacleLeft => match direction.axis() {
                        Axis::Vertical => {
                            queue.push_back(
                                (point + direction)
                                    .expect("ObstacleLeft should not be at edge of map"),
                            );
                            queue.push_back(
                                (point + Direction::Right)
                                    .expect("ObstacleLeft should not be at edge of map"),
                            );
                        }
                        Axis::Horizontal => queue.push_back(
                            (point + direction).expect("ObstacleLeft should not be at edge of map"),
                        ),
                    },
                    Tile::ObstacleRight => match direction.axis() {
                        Axis::Vertical => {
                            queue.push_back(
                                (point + direction)
                                    .expect("ObstacleRight should not be at edge of map"),
                            );
                            queue.push_back(
                                (point + Direction::Left)
                                    .expect("ObstacleRight should not be at edge of map"),
                            );
                        }
                        Axis::Horizontal => queue.push_back(
                            (point + direction)
                                .expect("ObstacleRight should not be at edge of map"),
                        ),
                    },
                    Tile::Wall => continue 'moves,
                }

                to_move.push((point, tile));
            }

            let to_move_points: HashSet<_> = to_move.iter().map(|(p, _)| *p).collect();

            for (point, tile) in to_move {
                let new_point = (point + direction).unwrap();
                *new_point.on(&map).unwrap().value_mut() = tile;
                if matches!(tile, Tile::Robot) {
                    robot_point = new_point;
                }
                if let Some(behind) = point - direction {
                    if !to_move_points.contains(&behind) {
                        *point.on(&map).unwrap().value_mut() = Tile::Empty;
                    }
                }
            }
        }

        #[cfg(test)]
        println!("\n{}", map);
        map
    }

    pub fn calculate(input: &str) -> usize {
        score(&evaluate(input))
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day15.txt");
            let map = evaluate(&input);
            assert_eq!(
                format!("{}", map).trim(),
                "
####################
##[].......[].[][]##
##[]...........[].##
##[]........[][][]##
##[]......[]....[]##
##..##......[]....##
##..[]............##
##..@......[].[][]##
##......[][]..[]..##
####################"
                    .trim()
            );
            assert_eq!(score(&map), 9021);
        }

        #[test]
        fn test_example_part2() {
            let input = aoc::example::example_string("day15_part2.txt");
            let map = evaluate(&input);
            assert_eq!(
                format!("{}", map).trim(),
                "
##############
##...[].##..##
##...@.[]...##
##....[]....##
##..........##
##..........##
##############"
                    .trim()
            )
        }

        #[test]
        fn test_score() {
            assert_eq!(
                score(
                    &"
##########
##...[]...
##........
"
                    .trim()
                    .parse()
                    .unwrap()
                ),
                105
            )
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
