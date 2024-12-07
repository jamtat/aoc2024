use std::str::FromStr;

use aoc2024::aoc;

struct Report(Vec<isize>);

impl Report {
    pub fn is_safe(&self) -> bool {
        let diffs = self.0.iter().zip(&self.0[1..]).map(|(a, b)| a - b);

        let mut last: Option<isize> = None;

        for d in diffs {
            if !(1..=3).contains(&d.abs()) {
                return false;
            }

            if let Some(last) = last {
                if last.signum() != d.signum() {
                    return false;
                }
            }
            last = Some(d);
        }

        true
    }
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.split(' ')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_safe() {
        assert!(Report(vec![7, 6, 4, 2, 1]).is_safe());
        assert!(!Report(vec![1, 2, 7, 8, 9]).is_safe());
        assert!(!Report(vec![9, 7, 6, 2, 1]).is_safe());
        assert!(!Report(vec![1, 3, 2, 4, 5]).is_safe());
        assert!(!Report(vec![8, 6, 4, 4, 1]).is_safe());
        assert!(Report(vec![1, 3, 6, 7, 9]).is_safe());
    }
}

mod part1 {
    use super::Report;

    fn parse_input(input: &str) -> Vec<Report> {
        input.lines().map(|s| s.parse().unwrap()).collect()
    }

    pub fn calculate(input: &str) -> usize {
        parse_input(input)
            .into_iter()
            .filter(Report::is_safe)
            .count()
    }

    #[cfg(test)]
    mod test {
        use super::calculate;
        use aoc2024::aoc;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day2.txt");

            assert_eq!(calculate(&input), 2);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();
    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
}
