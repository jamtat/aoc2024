use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Debug, Display, Write},
    hash::Hash,
    str::FromStr,
};

use aoc2024::aoc::{
    self,
    grid::{Direction, Grid, GridCell, Point},
};

pub type Map = Grid<Vec<Tile>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    Empty,
    Wall,
    Start,
    End,
    Overlay(Direction),
}

impl Tile {
    pub fn traversible(&self) -> bool {
        !matches!(self, Tile::Wall)
    }

    pub fn terminus(&self) -> bool {
        matches!(self, Tile::Start | Tile::End)
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
        })
    }
}

#[derive(Clone)]
struct State<'a> {
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

    pub fn point(&self) -> Point {
        self.cell.point()
    }

    pub fn tile(&self) -> Tile {
        *self.cell.value()
    }

    pub fn is_end(&self) -> bool {
        matches!(self.tile(), Tile::End)
    }
}

impl DState for State<'_> {
    type Position = (Point, Direction);

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
                out.push(State::new(self.direction, next, self.cost + BASE_COST));
            }
        }

        out.push(State::new(
            self.direction.turn_left(),
            self.cell,
            self.cost + TURN_COST,
        ));
        out.push(State::new(
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

trait DState: Sized + PartialOrd + Ord + PartialEq + Eq {
    type Position: Sized + PartialEq + Eq + Hash;

    fn cost(&self) -> usize;
    fn position(&self) -> Self::Position;
    fn next(&self) -> Vec<Self>;
}

fn djikstras<S, F>(start: S, is_end: F) -> Vec<Vec<S>>
where
    S: DState,
    F: Fn(&S) -> bool,
{
    let mut costs: HashMap<S::Position, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    costs.insert(start.position(), 0);
    heap.push(start);

    let mut paths: Vec<Vec<S>> = vec![];

    while let Some(state) = heap.pop() {
        let position = state.position();
        let cost = state.cost();

        if is_end(&state) {
            if let Some(existing_end_state) = paths.first() {
                let existing_end_state = existing_end_state.first().unwrap();
                if state.cost() > existing_end_state.cost() {
                    break;
                }
            }
            paths.push(vec![state]);
            continue;
        }

        if costs
            .get(&position)
            .map(|&existing_cost| cost > existing_cost)
            .unwrap_or(false)
        {
            continue;
        }

        for next in state.next() {
            let next_position = next.position();
            let existing_cost = *costs.get(&next_position).unwrap_or(&usize::MAX);
            let next_cost = next.cost();
            if next_cost < existing_cost {
                costs.insert(next_position, next_cost);
                heap.push(next);
            }
        }
    }

    paths
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let map = input.parse::<Map>().unwrap();

        let start_state = State::new(
            Direction::Right,
            map.find_by_value(|tile| matches!(tile, Tile::Start))
                .expect("Map must have a start"),
            0,
        );

        let end_states = djikstras(start_state, State::is_end);

        end_states.first().unwrap().first().unwrap().cost()
    }

    #[cfg(test)]
    mod test {
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

        #[test]
        fn test_input() {
            let input = aoc::cli::input_string("day16.txt");
            assert_eq!(calculate(&input), 143564);
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
            let input = aoc::example::example_string("day16.txt");
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
