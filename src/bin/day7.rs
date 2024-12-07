use std::iter;

use aoc2024::aoc;
use itertools::Itertools;

type Op = fn(usize, usize) -> usize;

struct Equation {
    result: usize,
    args: Vec<usize>,
}

impl Equation {
    pub fn possible(&self, ops: &'static [Op]) -> bool {
        self.evaluate(ops).contains(&self.result)
    }

    pub fn evaluate(&self, ops: &'static [Op]) -> impl Iterator<Item = usize> + '_ {
        evaluate(&self.args, ops)
    }
}

fn add(a: usize, b: usize) -> usize {
    a + b
}
fn mul(a: usize, b: usize) -> usize {
    a * b
}
fn concat(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}

fn evaluate<'a>(args: &'a [usize], ops: &'static [Op]) -> Box<dyn Iterator<Item = usize> + 'a> {
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

mod part1 {
    use super::*;

    pub fn calculate(equations: &[Equation]) -> usize {
        equations
            .iter()
            .filter_map(|e| e.possible(&[add, mul]).then_some(e.result))
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
            assert_eq!(calculate(&parse_input(&input)), 3749);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(equations: &[Equation]) -> usize {
        equations
            .iter()
            .filter_map(|e| e.possible(&[add, mul, concat]).then_some(e.result))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day7.txt");
            assert_eq!(calculate(&parse_input(&input)), 11387);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();
    let equations = parse_input(&input);

    println!("Part 1: {}", part1::calculate(&equations));
    println!("Part 2: {}", part2::calculate(&equations));
}
