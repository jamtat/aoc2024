use std::{fmt::Display, str::FromStr};

use aoc2024::aoc;

#[derive(Debug, PartialEq, Eq, Clone)]
enum StoneList {
    Stone(usize),
    Cons(Box<StoneList>, Box<StoneList>),
}

impl StoneList {
    pub fn cons(left: StoneList, right: StoneList) -> StoneList {
        StoneList::Cons(Box::new(left), Box::new(right))
    }
    pub fn from_slice(stones: &[usize]) -> Option<Self> {
        match stones {
            [] => None,
            [x] => Some(StoneList::Stone(*x)),
            [x, tail @ ..] => Some(StoneList::cons(
                StoneList::Stone(*x),
                StoneList::from_slice(tail).unwrap(),
            )),
        }
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
}

fn split_pow10(n: usize, pow10: u32) -> (usize, usize) {
    let div = 10usize.pow(pow10);
    (n / div, n % div)
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

impl Display for StoneList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoneList::Stone(stone) => write!(f, "{}", stone)?,
            StoneList::Cons(left, right) => write!(f, "{left} {right}")?,
        }

        Ok(())
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

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let mut stones: StoneList = input.parse().unwrap();
        for _ in 0..25 {
            stones.blink();
        }

        stones.len()
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
            let input = aoc::example::example_string("day11.txt");
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
