use aoc2024::aoc;
use std::collections::HashMap;

mod part1 {
    use super::*;

    fn possible(towels: &[String], pattern: &str, cache: &mut HashMap<String, bool>) -> bool {
        if !cache.contains_key(pattern) {
            let val = pattern.is_empty()
                || towels.iter().any(|towel| {
                    pattern.starts_with(towel) && possible(towels, &pattern[towel.len()..], cache)
                });
            cache.insert(pattern.to_string(), val);
        }
        *cache.get(pattern).unwrap()
    }

    pub fn calculate(input: &str) -> usize {
        let mut lines = input.lines();
        let towels: Vec<String> = lines
            .next()
            .unwrap()
            .split(", ")
            .map(str::to_string)
            .collect();

        lines.next();

        let patterns: Vec<String> = lines.map(str::to_string).collect();

        patterns
            .iter()
            .filter(|pattern| possible(&towels, pattern, &mut HashMap::new()))
            .count()
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use aoc2024::aoc;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day19.txt");
            assert_eq!(calculate(&input), 6);
        }
    }
}

mod part2 {
    use super::*;

    fn arrangements(towels: &[String], pattern: &str, cache: &mut HashMap<String, usize>) -> usize {
        if pattern.is_empty() {
            return 0;
        }
        if !cache.contains_key(pattern) {
            let count = towels
                .iter()
                .map(|towel| {
                    if pattern == towel {
                        1
                    } else if pattern.starts_with(towel) {
                        arrangements(towels, &pattern[towel.len()..], cache)
                    } else {
                        0
                    }
                })
                .sum();
            cache.insert(pattern.to_string(), count);
        }
        *cache.get(pattern).unwrap()
    }

    pub fn calculate(input: &str) -> usize {
        let mut lines = input.lines();
        let towels: Vec<String> = lines
            .next()
            .unwrap()
            .split(", ")
            .map(str::to_string)
            .collect();

        lines.next();

        let patterns: Vec<String> = lines.map(str::to_string).collect();

        patterns
            .iter()
            .map(|pattern| arrangements(&towels, pattern, &mut HashMap::new()))
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day19.txt");
            assert_eq!(calculate(&input), 16);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
