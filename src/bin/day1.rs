use aoc2024::aoc;

mod part1 {

    fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
        let pairs: Vec<_> = input
            .lines()
            .map(|l| {
                let mut l = l.split_ascii_whitespace();
                (
                    l.next().unwrap().parse::<usize>().unwrap(),
                    l.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .collect();

        (
            pairs.iter().map(|p| p.0).collect(),
            pairs.iter().map(|p| p.1).collect(),
        )
    }

    pub fn calculate(input: &str) -> usize {
        let (mut left, mut right) = parse_input(input);

        left.sort();
        right.sort();

        left.iter()
            .zip(right.iter())
            .map(|(l, r)| if l > r { l - r } else { r - l })
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use aoc2024::aoc;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day1.txt");

            assert_eq!(calculate(&input), 11);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();
    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
}
