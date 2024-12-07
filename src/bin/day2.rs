use std::str::FromStr;

use aoc2024::aoc;

struct Report(Vec<isize>);

impl Report {
    fn pairs(&self) -> impl Iterator<Item = (&isize, &isize)> + '_ {
        self.0.iter().zip(&self.0[1..]).to_owned()
    }

    fn diffs(&self) -> impl Iterator<Item = isize> + '_ {
        self.pairs().map(|(a, b)| a - b)
    }

    pub fn diff_safe(diff: isize, last_diff: Option<isize>) -> bool {
        (1..=3).contains(&diff.abs())
            && last_diff
                .map(|l| l.signum() == diff.signum())
                .unwrap_or(true)
    }

    pub fn is_safe(&self) -> bool {
        let mut last: Option<isize> = None;

        for d in self.diffs() {
            if !Self::diff_safe(d, last) {
                return false;
            }
            last = Some(d);
        }

        true
    }

    pub fn with_one_dropped(&self) -> impl Iterator<Item = Report> + '_ {
        (0..self.0.len()).map(|i| {
            Report(
                self.0[..i]
                    .iter()
                    .chain(self.0[i + 1..].iter())
                    .cloned()
                    .collect(),
            )
        })
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

fn parse_input(input: &str) -> Vec<Report> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        parse_input(input)
            .into_iter()
            .filter(Report::is_safe)
            .count()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day2.txt");

            assert_eq!(calculate(&input), 2);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        parse_input(input)
            .into_iter()
            .filter(|r| r.is_safe() || r.with_one_dropped().any(|r| r.is_safe()))
            .count()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day2.txt");

            assert_eq!(calculate(&input), 4);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();
    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
