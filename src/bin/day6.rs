use aoc::grid::Point;
use std::collections::HashSet;
use std::fmt::Display;

use aoc2024::aoc::{
    self,
    grid::{Direction, Grid},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstacle,
    Guard(Direction),
}

impl Tile {
    pub fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Obstacle => '#',
            Self::Guard(direction) => match direction {
                Direction::Up => '^',
                Direction::Down => 'v',
                Direction::Left => '<',
                Direction::Right => '>',
            },
        }
    }

    pub fn is_guard(&self) -> bool {
        matches!(self, Self::Guard(_))
    }

    pub fn is_obstacle(&self) -> bool {
        matches!(self, Self::Obstacle)
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Obstacle),
            '^' => Some(Self::Guard(Direction::Up)),
            'v' => Some(Self::Guard(Direction::Down)),
            '<' => Some(Self::Guard(Direction::Left)),
            '>' => Some(Self::Guard(Direction::Right)),
            _ => None,
        }
    }

    pub fn direction(&self) -> Option<Direction> {
        match self {
            Self::Guard(direction) => Some(*direction),
            _ => None,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

type Map = Grid<Vec<Tile>>;

fn parse_input(input: &str) -> (Map, Point, Direction) {
    let input = input.trim();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let tiles: Vec<_> = input.chars().filter_map(Tile::from_char).collect();
    assert!(width * height == tiles.len());

    let map = Map::new(width, height, tiles);

    let (point, direction) = {
        let mut guard = map
            .iter()
            .find(|cell| cell.value().is_guard())
            .expect("Could not find a guard in the map");
        let point = guard.point();
        let direction = guard.value().direction().unwrap();
        *guard.value_mut() = Tile::Empty;
        (point, direction)
    };

    (map, point, direction)
}

fn walk(map: &Map, start: &Point, direction: &Direction) -> Option<HashSet<Point>> {
    let mut loc = *start;
    let mut direction = *direction;
    let mut visited: HashSet<(Point, Direction)> = HashSet::new();

    while map.in_bounds_point(&loc) {
        visited.insert((loc, direction));
        let Some(cell) = map
            .cell_at_point(&loc)
            .map(|cell| cell.go(&direction))
            .flatten()
        else {
            break;
        };

        if cell.value().is_obstacle() {
            direction = direction.turn_right();
            if visited.contains(&(loc, direction)) {
                return None;
            }
            continue;
        } else {
            loc = cell.point();
        }
    }

    Some(visited.into_iter().map(|(point, _)| point).collect())
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let (map, start, direction) = parse_input(input);
        walk(&map, &start, &direction)
            .expect("Walk without cycles")
            .len()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = &aoc::example::example_string("day6.txt");
            assert_eq!(calculate(input), 41);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let (map, start, direction) = parse_input(input);
        println!(
            "Grid size: {}x{} ({})",
            map.width(),
            map.height(),
            map.len()
        );

        let initial_walk = walk(&map, &start, &direction).expect("Initial walk without cycles");
        println!("Cells visited initially {}", initial_walk.len());

        let mut cycles = 0;
        let mut i = 0;

        for point in initial_walk {
            i += 1;
            println!("Cell {i}");
            let mut cell = map
                .cell_at_point(&point)
                .expect("Cell must exist at walked point");

            if cell.value().is_obstacle() {
                continue;
            }
            *cell.value_mut() = Tile::Obstacle;

            if walk(&map, &start, &direction).is_none() {
                cycles += 1;
                println!("Found {cycles} cycles");
            }

            *cell.value_mut() = Tile::Empty;
        }

        cycles
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = &aoc::example::example_string("day6.txt");
            assert_eq!(calculate(input), 6);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input)); // 1789
}
