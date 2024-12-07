use aoc2024::aoc;
use std::collections::{HashMap, HashSet};

mod part1 {
    use super::*;

    fn page_valid(page: &[usize], rules_after: &HashMap<usize, HashSet<usize>>) -> bool {
        for i in 0..page.len() - 1 {
            let n = page[i];
            let rest = &page[i + 1..];
            if let Some(banned_pages) = rules_after.get(&n) {
                for nn in rest {
                    if banned_pages.contains(nn) {
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn calculate(input: &str) -> usize {
        let (rules, pages) = input.split_once("\n\n").unwrap();

        let (_rules_before, rules_after): (
            HashMap<usize, HashSet<usize>>,
            HashMap<usize, HashSet<usize>>,
        ) = rules
            .trim()
            .lines()
            .map(|l| -> (usize, usize) {
                let (before, after) = l.split_once('|').unwrap();
                (before.parse().unwrap(), after.parse().unwrap())
            })
            .fold(
                Default::default(),
                |(mut before_acc, mut after_acc), (before, after)| {
                    before_acc.entry(before).or_default().insert(after);
                    after_acc.entry(after).or_default().insert(before);
                    (before_acc, after_acc)
                },
            );

        let pages: Vec<Vec<usize>> = pages
            .trim()
            .lines()
            .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
            .collect();

        pages
            .iter()
            .filter(|page| page_valid(page, &rules_after))
            .map(|page| page[page.len() / 2])
            .sum()
    }

    #[cfg(test)]
    mod test {

        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day5.txt");

            assert_eq!(calculate(&input), 143);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
}
