use aoc2024::aoc::{
    self,
    algo::djikstra::DjikstraState,
    grid::{Grid, GridCell, Point},
};
use std::fmt::{Display, Write};

type Map = Grid<Vec<Tile>>;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Obstruction,
    Overlay,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Tile::Empty => '.',
            Tile::Obstruction => '#',
            Tile::Overlay => 'O',
        })
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

mod state {
    use aoc::grid::Direction;

    use super::*;

    pub struct MapDState<'a> {
        cell: GridCell<'a, Vec<Tile>>,
        cost: usize,
    }

    impl<'a> MapDState<'a> {
        pub fn new(cell: GridCell<'a, Vec<Tile>>, cost: usize) -> Self {
            Self { cell, cost }
        }

        pub fn point(&self) -> Point {
            self.cell.point()
        }
    }

    impl PartialOrd for MapDState<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for MapDState<'_> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then(other.point().cmp(&self.point()))
        }
    }

    impl PartialEq for MapDState<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.point() == other.point() && self.cost == other.cost
        }
    }

    impl Eq for MapDState<'_> {}

    impl DjikstraState for MapDState<'_> {
        type Position = Point;
        type Cost = usize;

        fn cost(&self) -> Self::Cost {
            self.cost
        }

        fn position(&self) -> Self::Position {
            self.point()
        }

        fn next(&self) -> Vec<Self> {
            Direction::all()
                .iter()
                .filter_map(|direction| {
                    self.cell
                        .go(direction)
                        .filter(|cell| *cell.value() == Tile::Empty)
                        .map(|cell| Self::new(cell, self.cost + 1))
                })
                .collect()
        }
    }
}

mod part1 {
    use super::*;
    use aoc::algo::djikstra::Djikstra;
    use state::MapDState;

    fn parse_input(input: &str) -> Vec<Point> {
        input
            .lines()
            .filter_map(|l| {
                let (x, y) = l.split_once(',')?;
                Some((x.parse::<usize>().ok()?, y.parse::<usize>().ok()?).into())
            })
            .collect()
    }

    pub fn calculate(input: &str, width: usize, height: usize, count: usize) -> usize {
        let points = &parse_input(input);
        let map = Map::default(width, height);

        for point in &points[0..count] {
            *point.on(&map).unwrap().value_mut() = Tile::Obstruction;
        }

        println!("{map}");

        let start = MapDState::new(map.cell_at(0, 0).unwrap(), 0);
        let end_point = Point::new(width - 1, height - 1);
        let is_end = |state: &MapDState| state.point() == end_point;

        let end_state = Djikstra::new(start, is_end).next().unwrap();

        for point in end_state.path() {
            *point.on(&map).unwrap().value_mut() = Tile::Overlay;
        }

        println!("\n\n{map}");

        end_state.cost()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day18.txt");
            assert_eq!(calculate(&input, 7, 7, 12), 22);
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
            let input = aoc::example::example_string("day18.txt");
            assert_eq!(calculate(&input), 0);
        }
    }
}
*/
fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input, 71, 71, 1024));
    // println!("Part 2: {}", part2::calculate(&input));
}
