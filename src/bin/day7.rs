use std::iter;

use aoc2024::aoc;
use itertools::Itertools;

struct Equation {
    result: usize,
    args: Vec<usize>,
}

fn parse_input(s: &str) -> Vec<Equation> {
    s.lines()
        .map(|l| {
            let (result, args) = l.split_once(": ").unwrap();
            let result = result.parse().unwrap();
            let args = args.split(' ').map(|s| s.parse().unwrap()).collect();

            Equation { result, args }
        })
        .collect()
}

fn add(a: usize, b: usize) -> usize {
    a + b
}
fn mul(a: usize, b: usize) -> usize {
    a * b
}

fn evaluate<'a>(
    args: &'a [usize],
    ops: &'static [fn(usize, usize) -> usize],
) -> Box<dyn Iterator<Item = usize> + 'a> {
    match args {
        [] => Box::new(iter::empty()),
        [x] => Box::new(iter::once(*x)),
        [head @ .., x] => Box::new(
            evaluate(head, ops)
                .map(move |n| ops.iter().map(move |op| op(n, *x)))
                .flatten(),
        ),
    }
}

mod part1 {
    use super::*;

    pub fn possible(equation: &Equation) -> bool {
        evaluate(&equation.args, &[add, mul]).contains(&equation.result)
    }

    pub fn calculate(input: &str) -> usize {
        let equations = parse_input(input);

        equations
            .iter()
            .filter_map(|e| possible(e).then_some(e.result))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day7.txt");
            for val in evaluate(&[81, 40, 27], &[add, mul]) {
                println!("{val}");
            }
            assert_eq!(calculate(&input), 3749);
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
            let input = aoc::example::example_string("day7.txt");
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
