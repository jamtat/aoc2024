use aoc2024::aoc::{self};

mod map {
    use std::{
        fmt::{Display, Write},
        str::FromStr,
    };

    use aoc2024::aoc::grid::{Direction, Grid};

    pub type Map = Grid<Vec<Tile>>;

    #[allow(dead_code)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Tile {
        Empty,
        Wall,
        Start,
        End,
        Overlay(Direction),
        Path,
    }

    impl Tile {
        pub fn traversible(&self) -> bool {
            !matches!(self, Tile::Wall)
        }
    }

    impl FromStr for Tile {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "." => Self::Empty,
                "#" => Self::Wall,
                "S" => Self::Start,
                "E" => Self::End,
                _ => Err(s.to_string())?,
            })
        }
    }

    impl Display for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_char(match self {
                Tile::Empty => ' ',
                Tile::Wall => '#',
                Tile::Start => 'S',
                Tile::End => 'E',
                Tile::Overlay(direction) => direction.char(),
                Tile::Path => 'O',
            })
        }
    }
}

mod state {
    use super::map::Tile;
    use aoc2024::aoc::{
        algo::djikstra::DjikstraState,
        grid::{Direction, GridCell, Point},
    };
    use std::fmt::Debug;

    #[derive(Clone)]
    pub struct State<'a> {
        direction: Direction,
        cell: GridCell<'a, Vec<Tile>>,
        cost: usize,
    }

    impl<'a> State<'a> {
        pub fn new(direction: Direction, cell: GridCell<'a, Vec<Tile>>, cost: usize) -> Self {
            Self {
                direction,
                cell,
                cost,
            }
        }

        fn point(&self) -> Point {
            self.cell.point()
        }

        fn tile(&self) -> Tile {
            *self.cell.value()
        }

        pub fn is_end(&self) -> bool {
            matches!(self.tile(), Tile::End)
        }
    }

    impl DjikstraState for State<'_> {
        type Position = (Point, Direction);
        type Cost = usize;

        fn cost(&self) -> usize {
            self.cost
        }

        fn position(&self) -> Self::Position {
            (self.point(), self.direction)
        }

        fn next(&self) -> Vec<Self> {
            let mut out = vec![];
            const BASE_COST: usize = 1;
            const TURN_COST: usize = 1000;

            if let Some(next) = self.cell.go(&self.direction) {
                if next.value().traversible() {
                    out.push(Self::new(self.direction, next, self.cost + BASE_COST));
                }
            }

            out.push(Self::new(
                self.direction.turn_left(),
                self.cell,
                self.cost + TURN_COST,
            ));
            out.push(Self::new(
                self.direction.turn_right(),
                self.cell,
                self.cost + TURN_COST,
            ));

            out
        }
    }

    impl Debug for State<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("State")
                .field("direction", &self.direction)
                .field("point", &self.point().tuple())
                .field("cost", &self.cost)
                // .field("last", &self.last)
                .finish()
        }
    }

    impl Ord for State<'_> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| other.direction.cmp(&self.direction))
                .then_with(|| other.point().cmp(&self.point()))
        }
    }

    impl PartialOrd for State<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for State<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.direction == other.direction
                && self.point() == other.point()
                && self.cost == other.cost
        }
    }

    impl Eq for State<'_> {}
}

mod part1 {
    use crate::{
        map::{Map, Tile},
        state::State,
    };
    use aoc2024::aoc::{algo::djikstra::Djikstra, grid::Direction};

    pub fn calculate(input: &str) -> usize {
        let map = input.parse::<Map>().unwrap();

        let start_state = State::new(
            Direction::Right,
            map.find_by_value(|tile| matches!(tile, Tile::Start))
                .expect("Map must have a start"),
            0,
        );

        Djikstra::new(start_state, State::is_end)
            .next()
            .expect("Should find an end state")
            .cost()
    }

    #[cfg(test)]
    mod test {
        use aoc2024::aoc;

        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day16.txt");
            assert_eq!(calculate(&input), 7036);
        }

        #[test]
        fn test_example_2() {
            let input = aoc::example::example_string("day16_2.txt");
            assert_eq!(calculate(&input), 11048);
        }

        // #[test]
        // fn test_input() {
        //     let input = aoc::cli::input_string("day16.txt");
        //     assert_eq!(calculate(&input), 143564);
        // }
    }
}

mod part2 {
    use crate::{
        map::{Map, Tile},
        state::State,
    };
    use aoc2024::aoc::{
        algo::djikstra::{Djikstra, QueueState},
        grid::Direction,
    };
    use std::collections::HashSet;

    #[allow(dead_code)]
    fn apply(map: &Map, state: &QueueState<State>) -> Map {
        let map = map.clone();

        for (point, direction) in state.path() {
            *point.on(&map).unwrap().value_mut() = Tile::Overlay(*direction);
        }
        map
    }

    pub fn calculate(input: &str) -> usize {
        let map = input.parse::<Map>().unwrap();

        let start_state = State::new(
            Direction::Right,
            map.find_by_value(|tile| matches!(tile, Tile::Start))
                .expect("Map must have a start"),
            0,
        );

        let end_states = Djikstra::new(start_state, State::is_end).collect::<Vec<_>>();

        let points: HashSet<_> = end_states
            .iter()
            .flat_map(|s| {
                s.path()
                    .iter()
                    .map(|&(point, _direction)| point)
                    .collect::<Vec<_>>()
            })
            .collect();

        #[cfg(test)]
        {
            println!("Found {} end states", end_states.len());
            for state in &end_states {
                println!("\n\nCost: {}\n{}\n", state.cost(), apply(&map, state));
            }
            for point in &points {
                *point.on(&map).unwrap().value_mut() = Tile::Path;
            }
            println!("{}", map);
        }

        points.len()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use aoc2024::aoc;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day16.txt");
            assert_eq!(calculate(&input), 45);
        }

        #[test]
        fn test_example_2() {
            let input = aoc::example::example_string("day16_2.txt");
            assert_eq!(calculate(&input), 64);
        }

        // #[test]
        // fn test_input() {
        //     let input = aoc::cli::input_string("day16.txt");
        //     assert_eq!(calculate(&input), 593);
        // }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
