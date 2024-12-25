use std::{collections::HashMap, fmt::Display, str::FromStr};

use aoc2024::aoc;

fn split_pow10(n: usize, pow10: u32) -> (usize, usize) {
    let div = 10usize.pow(pow10);
    (n / div, n % div)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Blink {
    One(usize),
    Two(usize, usize),
}

fn blink(n: usize) -> Blink {
    if n == 0 {
        return Blink::One(1);
    }

    let pow10 = n.ilog10() + 1;

    if pow10 % 2 == 0 {
        let (left, right) = split_pow10(n, pow10 / 2);
        Blink::Two(left, right)
    } else {
        Blink::One(n * 2024)
    }
}

#[derive(Clone)]
struct StoneCounts(HashMap<usize, usize>);

impl StoneCounts {
    pub fn from_slice(stones: &[usize]) -> Self {
        Self(stones.iter().fold(Default::default(), |mut acc, stone| {
            *acc.entry(*stone).or_default() += 1;
            acc
        }))
    }

    pub fn blink(&self) -> Self {
        let mut out = HashMap::default();

        for (&stone, &count) in &self.0 {
            match blink(stone) {
                Blink::One(stone) => *out.entry(stone).or_default() += count,
                Blink::Two(left, right) => {
                    *out.entry(left).or_default() += count;
                    *out.entry(right).or_default() += count;
                }
            }
        }

        Self(out)
    }

    pub fn blink_n(&self, n: usize) -> Self {
        (0..n).fold(self.clone(), |stones, _| stones.blink())
    }

    pub fn len(&self) -> usize {
        self.0.values().sum()
    }
}

impl FromStr for StoneCounts {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .split_ascii_whitespace()
            .map(|s| s.parse())
            .collect::<Result<Vec<usize>, _>>()
            .map_err(|_| ())?;

        Ok(Self::from_slice(&stones))
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let stones: StoneCounts = input.parse().unwrap();
        stones.blink_n(25).len()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day11.txt");
            assert_eq!(calculate(&input), 55312);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let stones: StoneCounts = input.parse().unwrap();
        stones.blink_n(75).len()
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}

#[allow(dead_code)]
mod slow_but_fun {
    use super::*;
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum StoneList {
        Stone(usize),
        Cons(Box<StoneList>, Box<StoneList>),
    }

    impl StoneList {
        pub fn cons(left: StoneList, right: StoneList) -> StoneList {
            StoneList::Cons(Box::new(left), Box::new(right))
        }
        pub fn from_slice(stones: &[usize]) -> Option<Self> {
            Some(match stones {
                [] => None?,
                [x] => StoneList::Stone(*x),
                [x, tail @ ..] => {
                    StoneList::cons(StoneList::Stone(*x), StoneList::from_slice(tail).unwrap())
                }
            })
        }

        pub fn len(&self) -> usize {
            match self {
                StoneList::Stone(_) => 1,
                StoneList::Cons(left, right) => left.len() + right.len(),
            }
        }

        pub fn blink(&mut self) {
            match self {
                StoneList::Stone(n) => {
                    if *n == 0 {
                        *n = 1;
                        return;
                    }

                    let pow10 = n.ilog10() + 1;

                    if pow10 % 2 == 0 {
                        let (left, right) = split_pow10(*n, pow10 / 2);
                        *self = StoneList::cons(StoneList::Stone(left), StoneList::Stone(right));
                    } else {
                        *n *= 2024;
                    }
                }

                StoneList::Cons(left, right) => {
                    left.blink();
                    right.blink();
                }
            }
        }

        pub fn blink_n(&mut self, n: usize) {
            for _ in 0..n {
                self.blink();
            }
        }
    }

    impl FromStr for StoneList {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let values = s
                .split_ascii_whitespace()
                .map(|n| n.parse())
                .collect::<Result<Vec<usize>, _>>()
                .map_err(|_| ())?;

            StoneList::from_slice(&values).ok_or(())
        }
    }

    impl Display for StoneList {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                StoneList::Stone(stone) => write!(f, "{}", stone)?,
                StoneList::Cons(left, right) => write!(f, "{left} {right}")?,
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_split_pow10() {
            let mut stone = StoneList::Stone(1000);
            stone.blink();
            assert_eq!(
                stone,
                StoneList::cons(StoneList::Stone(10), StoneList::Stone(0))
            );
        }
    }
}
