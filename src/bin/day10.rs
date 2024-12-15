use aoc::grid::{Grid, GridCell};
use aoc2024::aoc;
use std::collections::{HashSet, VecDeque};

type Map = Grid<Vec<u8>>;

mod part1 {

    use aoc::grid::{Direction, Point};

    use super::*;

    fn trailheads(start: &GridCell<'_, Vec<u8>>) -> usize {
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

    pub fn calculate(input: &str) -> usize {
        let grid = input.parse::<Map>().unwrap();

        grid.iter()
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
            assert_eq!(calculate(&input), 36);
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
            let input = aoc::example::example_string("day10.txt");
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
