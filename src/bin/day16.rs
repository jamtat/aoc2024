use std::{
    collections::{BinaryHeap, HashMap},
    fmt::{Debug, Display, Write},
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
    last: Option<Box<State<'a>>>,
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

impl<'a> State<'a> {
    pub fn new(
        direction: Direction,
        cell: GridCell<'a, Vec<Tile>>,
        cost: usize,
        last: Option<Box<Self>>,
    ) -> Self {
        Self {
            direction,
            cell,
            cost,
            last,
        }
    }

    pub fn point(&self) -> Point {
        self.cell.point()
    }

    pub fn boxed(&self) -> Box<Self> {
        self.clone().into()
    }

    pub fn len(&self) -> usize {
        match &self.last {
            Some(last) => last.len() + 1,
            None => 1,
        }
    }

    pub fn same_direction(&self) -> bool {
        if let Some(last) = &self.last {
            last.direction == self.direction
        } else {
            false
        }
    }

    pub fn position(&self) -> StatePosition {
        (self.point(), self.direction)
    }
}

impl<'a> IntoIterator for State<'a> {
    type Item = <StateIter<'a> as Iterator>::Item;

    type IntoIter = StateIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        StateIter {
            next_state: Some(self),
        }
    }
}

struct StateIter<'a> {
    next_state: Option<State<'a>>,
}

impl<'a> Iterator for StateIter<'a> {
    type Item = State<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.next_state.take();
        self.next_state = ret.clone().and_then(|state| state.last.map(|boxed| *boxed));
        ret
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.same_direction().cmp(&self.same_direction()))
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

type StatePosition = (Point, Direction);

fn next_states(
    state @ State {
        direction,
        cell,
        cost,
        last: _,
    }: State<'_>,
) -> Vec<State<'_>> {
    let mut out = vec![];
    const BASE_COST: usize = 1;
    const TURN_COST: usize = 1000;

    if let Some(next) = cell.go(&direction) {
        if next.value().traversible() {
            out.push(State::new(
                direction,
                next,
                cost + BASE_COST,
                Some(state.boxed()),
            ));
        }
    }
    let left = direction.turn_left();
    let right = direction.turn_right();
    // Could potentally need to turn backwards depending on the start position
    // but after checking examples and input this is not necessary

    out.push(State::new(
        left,
        cell,
        cost + TURN_COST,
        Some(state.boxed()),
    ));
    out.push(State::new(
        right,
        cell,
        cost + TURN_COST,
        Some(state.boxed()),
    ));

    out.sort_by_key(|state| state.cost);
    out
}

fn djikstras(map: &Map) -> Option<State> {
    let start = map
        .find_by_value(|tile| matches!(tile, Tile::Start))
        .expect("Map must have a start");

    let end_point = map
        .find_by_value(|tile| matches!(tile, Tile::End))
        .expect("Map must have an end")
        .point();

    let mut costs: HashMap<StatePosition, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    {
        let start_state = State::new(Direction::Right, start, 0, None);
        costs.insert(start_state.position(), 0);
        heap.push(start_state);
    }

    while let Some(
        state @ State {
            direction: _,
            cell: _,
            cost,
            last: _,
        },
    ) = heap.pop()
    {
        let point = state.point();
        let position = state.position();
        if point == end_point {
            return Some(state);
        }

        if costs
            .get(&position)
            .map(|&existing_cost| cost > existing_cost)
            .unwrap_or(false)
        {
            continue;
        }

        for next in next_states(state) {
            let next_position = next.position();
            let existing_cost = *costs.get(&next_position).unwrap_or(&usize::MAX);
            if next.cost < existing_cost {
                costs.insert(next_position, next.cost);
                heap.push(next);
            }
        }
    }

    None
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let map = input.parse::<Map>().unwrap();
        let state = djikstras(&map).expect("Should find a shortest path to the end");
        let cost = state.cost;
        println!("Path len: {}", state.len());
        // #[cfg(test)]
        {
            for history in state.clone().into_iter().collect::<Vec<_>>().iter().rev() {
                println!("{:?}", history);
            }
            for mut path in state {
                if path.cell.value().terminus() {
                    continue;
                }
                *path.cell.value_mut() = Tile::Overlay(path.direction);
            }
            println!("{}", map);
        }

        cost
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
