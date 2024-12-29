use aoc2024::aoc;

mod part1 {
    use std::collections::HashMap;

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

        println!("Towels: {:?}", towels);
        println!("Patterns: {:?}", patterns);

        let mut cache: HashMap<String, bool> = HashMap::new();

        patterns
            .iter()
            .filter(|pattern| possible(&towels, pattern, &mut cache))
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
            let input = aoc::example::example_string("day19.txt");
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
