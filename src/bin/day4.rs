use aoc2024::aoc;

mod part1 {
    use std::ops::Index;

    use aoc::grid::{Direction, Grid, GridCell};

    use super::*;

    fn parse_input(input: &str) -> Grid<Vec<char>> {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let items: Vec<char> = input.lines().flat_map(|s| s.chars()).collect();
        assert!(items.len() == width * height);
        Grid::new(width, height, items)
    }

    fn apply_step<'a, T: Index<usize>>(
        cell: GridCell<'a, T>,
        step: &[Direction],
    ) -> Option<GridCell<'a, T>> {
        let mut cell = cell;
        for direction in step {
            cell = cell.go(direction)?;
        }
        Some(cell)

        // step.iter().fold(Some(cell), |cell, direction| {
        //     cell.map(|c| c.go(direction)).flatten()
        // })
    }

    pub fn calculate(input: &str) -> usize {
        let grid = parse_input(input);
        #[cfg(test)]
        {
            println!("{grid}");
        }
        let mut count = 0;

        static STEPS: &[&[Direction]] = &[
            &[Direction::UP],
            &[Direction::DOWN],
            &[Direction::LEFT],
            &[Direction::RIGHT],
            &[Direction::UP, Direction::LEFT],
            &[Direction::UP, Direction::RIGHT],
            &[Direction::DOWN, Direction::LEFT],
            &[Direction::DOWN, Direction::RIGHT],
        ];

        for x_cell in grid.iter().filter(|&c| *c == 'X') {
            'directions: for step in STEPS {
                let mut cell = x_cell;
                for xmas in ['M', 'A', 'S'] {
                    if let Some(c) = apply_step(cell, step) {
                        if *c != xmas {
                            continue 'directions;
                        }
                        cell = c;
                    } else {
                        continue 'directions;
                    }
                }
                count += 1;
            }
        }

        count
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day4.txt");
            assert_eq!(calculate(&input), 18);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
}
