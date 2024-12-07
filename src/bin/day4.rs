use aoc::grid::{Direction, Grid, Step};
use aoc2024::aoc;

fn parse_input(input: &str) -> Grid<Vec<char>> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let items: Vec<char> = input.lines().flat_map(|s| s.chars()).collect();
    assert!(items.len() == width * height);
    Grid::new(width, height, items)
}

mod part1 {
    use super::*;

    pub fn calculate(grid: &Grid<Vec<char>>) -> usize {
        use Direction as D;
        #[cfg(test)]
        {
            println!("{grid}");
        }
        let mut count = 0;

        static STEPS: &[&Step] = &[
            &[D::UP],
            &[D::DOWN],
            &[D::LEFT],
            &[D::RIGHT],
            &[D::UP, D::LEFT],
            &[D::UP, D::RIGHT],
            &[D::DOWN, D::LEFT],
            &[D::DOWN, D::RIGHT],
        ];

        for x_cell in grid.iter().filter(|&c| *c == 'X') {
            'directions: for step in STEPS {
                let mut cell = x_cell;
                for xmas in ['M', 'A', 'S'] {
                    if let Some(c) = cell.step(step) {
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
            assert_eq!(calculate(&parse_input(&input)), 18);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(grid: &Grid<Vec<char>>) -> usize {
        use Direction as D;

        /*
           Find all of the 'A's and check whether it forms "MAS" or "SAM" on each diagonal
           e.g. M.S
                .A.
                M.S

        */
        static UL: &Step = &[D::UP, D::LEFT];
        static UR: &Step = &[D::UP, D::RIGHT];
        static DL: &Step = &[D::DOWN, D::LEFT];
        static DR: &Step = &[D::DOWN, D::RIGHT];

        grid.iter()
            .filter(|&cell| *cell == 'A')
            .filter_map(|cell| -> Option<()> {
                let ul = *cell.step(UL)?;
                let ur = *cell.step(UR)?;
                let dl = *cell.step(DL)?;
                let dr = *cell.step(DR)?;

                match (ul, dr) {
                    ('M', 'S') | ('S', 'M') => match (ur, dl) {
                        ('M', 'S') | ('S', 'M') => Some(()),
                        _ => None,
                    },
                    _ => None,
                }
            })
            .count()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day4.txt");
            assert_eq!(calculate(&parse_input(&input)), 9);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();
    let grid = parse_input(&input);

    println!("Part 1: {}", part1::calculate(&grid));
    println!("Part 2: {}", part2::calculate(&grid));
}
