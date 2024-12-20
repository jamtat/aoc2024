use aoc2024::{aoc, point2d::Point2D};
use std::collections::{HashMap, HashSet};

#[allow(clippy::type_complexity)]
fn parse_input(
    input: &str,
) -> (
    impl Fn(&Point2D<isize>) -> bool,
    HashMap<char, Vec<Point2D<isize>>>,
) {
    let width = input.lines().next().unwrap().len() as isize;
    let height = input.lines().count() as isize;

    let frequency_points: HashMap<char, Vec<Point2D<isize>>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let y = y as isize;
            line.chars().enumerate().filter_map(move |(x, c)| {
                let x = x as isize;
                (c != '.').then_some((c, Point2D::new(x, y)))
            })
        })
        .fold(HashMap::new(), |mut acc, (c, point)| {
            acc.entry(c).or_default().push(point);
            acc
        });

    (
        move |p: &Point2D<isize>| p.x >= 0 && p.x < width && p.y >= 0 && p.y < height,
        frequency_points,
    )
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let (in_bounds, frequency_points) = parse_input(input);

        let mut antinodes: HashSet<Point2D<isize>> = HashSet::new();

        for (_, points) in frequency_points {
            for i in 0..points.len() - 1 {
                for j in i + 1..points.len() {
                    let p1 = points[i];
                    let p2 = points[j];

                    let diff = p2 - p1;
                    let a1 = p1 - diff;
                    let a2 = p2 + diff;

                    antinodes.insert(a1);
                    antinodes.insert(a2);
                }
            }
        }
        antinodes.into_iter().filter(in_bounds).count()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day8.txt");
            assert_eq!(calculate(&input), 14);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let (in_bounds, frequency_points) = parse_input(input);

        let mut antinodes: HashSet<Point2D<isize>> = HashSet::new();

        for (_, points) in frequency_points {
            for i in 0..points.len() - 1 {
                for j in i + 1..points.len() {
                    let p1 = points[i];
                    let p2 = points[j];

                    let diff = p2 - p1;

                    for k in 0.. {
                        let a = p1 - (diff * k);
                        if in_bounds(&a) {
                            antinodes.insert(a);
                        } else {
                            break;
                        }
                    }
                    for k in 0.. {
                        let a = p2 + (diff * k);
                        if in_bounds(&a) {
                            antinodes.insert(a);
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        antinodes.len()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day8.txt");
            assert_eq!(calculate(&input), 34);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
