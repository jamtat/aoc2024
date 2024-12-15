use aoc::grid::{Direction, Grid, GridCell, Point};
use aoc2024::aoc;
use std::collections::{HashSet, VecDeque};

type Map = Grid<Vec<u8>>;
type MapCell<'a> = GridCell<'a, Vec<u8>>;

mod part1 {
    use super::*;

    fn trailheads(start: &MapCell) -> usize {
        let mut checked = HashSet::<Point>::new();
        let mut queue = VecDeque::<GridCell<'_, _>>::new();
        queue.push_back(*start);
        let mut count = 0;

        while let Some(cell) = queue.pop_front() {
            let value = *cell.value();
            for cell in Direction::all()
                .iter()
                .filter_map(|direction| cell.go(direction))
            {
                let v = *cell.value();
                let p = cell.point();
                if checked.contains(&p) {
                    continue;
                }
                if v == value + 1 {
                    if v == 9 {
                        count += 1;
                    } else {
                        queue.push_back(cell);
                    }
                    checked.insert(p);
                }
            }
        }

        count
    }

    pub fn calculate(map: &Map) -> usize {
        map.iter()
            .filter(|cell| *cell.value() == 0)
            .map(|cell| trailheads(&cell))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day10.txt");
            assert_eq!(calculate(&input.parse().unwrap()), 36);
        }
    }
}

mod part2 {
    use super::*;

    fn rating(start: &MapCell) -> usize {
        let mut queue = VecDeque::<GridCell<'_, _>>::new();
        queue.push_back(*start);
        let mut rating = 0;

        while let Some(cell) = queue.pop_front() {
            let value = *cell.value();
            for cell in Direction::all()
                .iter()
                .filter_map(|direction| cell.go(direction))
            {
                let v = *cell.value();
                if v == value + 1 {
                    if v == 9 {
                        rating += 1;
                    } else {
                        queue.push_back(cell);
                    }
                }
            }
        }

        rating
    }

    pub fn calculate(map: &Map) -> usize {
        map.iter()
            .filter(|cell| *cell.value() == 0)
            .map(|cell| rating(&cell))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day10.txt");
            assert_eq!(calculate(&input.parse().unwrap()), 81);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();
    let map: Map = input.parse().expect("Could not parse map");
    println!("Part 1: {}", part1::calculate(&map));
    println!("Part 2: {}", part2::calculate(&map));
}
