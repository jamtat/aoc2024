use aoc2024::aoc::{
    self,
    algo::djikstra::{Djikstra, DjikstraState},
    grid::{Direction, Grid, GridCell, Point},
};
use std::{collections::HashMap, fmt::Display, str::FromStr};

mod map {
    use super::*;

    pub type Map = Grid<Vec<Tile>>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Tile {
        Empty,
        Obstacle,
        Start,
        End,
    }

    impl FromStr for Tile {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "." => Self::Empty,
                "#" => Self::Obstacle,
                "S" => Self::Start,
                "E" => Self::End,
                _ => Err(s.to_string())?,
            })
        }
    }

    impl Display for Tile {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Tile::Empty => '.',
                    Tile::Obstacle => '#',
                    Tile::Start => 'S',
                    Tile::End => 'E',
                }
            )
        }
    }
}

mod state {
    use super::*;
    use map::Tile;

    #[derive(Clone)]
    pub struct State<'a> {
        cell: GridCell<'a, Vec<Tile>>,
        cost: usize,
    }

    impl<'a> State<'a> {
        pub fn new(cell: GridCell<'a, Vec<Tile>>) -> Self {
            Self { cell, cost: 0 }
        }

        fn add(&self, cell: GridCell<'a, Vec<Tile>>) -> Self {
            Self {
                cell,
                cost: self.cost + 1,
            }
        }

        pub fn point(&self) -> Point {
            self.cell.point()
        }

        pub fn is_end(&self) -> bool {
            *self.cell.value() == Tile::End
        }
    }

    impl DjikstraState for State<'_> {
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
                .filter_map(|direction| self.cell.go(direction))
                .filter(|cell| *cell.value() != Tile::Obstacle)
                .map(|cell| self.add(cell))
                .collect()
        }
    }

    impl Ord for State<'_> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.point().cmp(&other.point()))
        }
    }

    impl PartialOrd for State<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for State<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.point() == other.point() && self.cost == other.cost
        }
    }

    impl Eq for State<'_> {}
}

use map::{Map, Tile};
use state::State;

pub fn calculate(input: &str, saving_target: usize, max_cheats: usize) -> usize {
    let map = input.parse::<Map>().unwrap();
    let start_cell = map.find_by_value(|tile| tile == Tile::Start).unwrap();

    // println!("Initial\n:{}\n", map);
    let base = Djikstra::new([State::new(start_cell)], State::is_end)
        .next()
        .unwrap();

    let mut savings = HashMap::<usize, usize>::new();
    let base_history = base.history().to_vec();

    // println!("Base cost: {base_cost}");

    for i in 0..base_history.len() - max_cheats {
        let start_state = &base_history[i];
        let test_states = &base_history[i + max_cheats + 1..];

        for end_state in test_states {
            if end_state.cost() < start_state.cost() {
                panic!("Wat")
            }
            let distance = start_state.point().manhattan_distance(&end_state.point());
            if distance > max_cheats {
                continue;
            }
            let saving = end_state.cost() - start_state.cost() - distance;
            *savings.entry(saving).or_insert(0) += 1;
        }
    }

    // #[cfg(test)]
    // {
    //     let mut saving_list = savings.iter().map(|(&k, &v)| (k, v)).collect::<Vec<_>>();
    //     saving_list.sort();

    //     for (saving, count) in saving_list {
    //         if count == 1 {
    //             println!("There is one cheat that saves {saving} picoseconds");
    //         } else {
    //             println!("There are {count} cheats that save {saving} picoseconds");
    //         }
    //     }
    // }

    savings
        .iter()
        .filter_map(|(&saving, &count)| (saving >= saving_target).then_some(count))
        .sum()
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", calculate(&input, 100, 2));
    println!("Part 2: {}", calculate(&input, 100, 20));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = aoc::example::example_string("day20.txt");
        assert_eq!(calculate(&input, 20, 2), 5);
    }

    #[test]
    fn test_example_2() {
        let input = aoc::example::example_string("day20.txt");
        assert_eq!(calculate(&input, 2, 2), 44);
    }
}
