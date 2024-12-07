use aoc2024::aoc;
use std::collections::{HashMap, HashSet};

fn update_valid(update: &[usize], rules_after: &HashMap<usize, HashSet<usize>>) -> bool {
    for i in 0..update.len() - 1 {
        let page = update[i];
        let rest = &update[i + 1..];
        if let Some(banned_pages) = rules_after.get(&page) {
            for nn in rest {
                if banned_pages.contains(nn) {
                    return false;
                }
            }
        }
    }

    true
}
struct Input {
    pub _rules_before: HashMap<usize, HashSet<usize>>,
    pub rules_after: HashMap<usize, HashSet<usize>>,
    pub updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Input {
    let (rules, updates) = input.split_once("\n\n").unwrap();

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

    let updates: Vec<Vec<usize>> = updates
        .trim()
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    Input {
        _rules_before,
        rules_after,
        updates,
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &Input) -> usize {
        input
            .updates
            .iter()
            .filter(|update| update_valid(update, &input.rules_after))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day5.txt");

            assert_eq!(calculate(&parse_input(&input)), 143);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(input: &Input) -> usize {
        input
            .updates
            .iter()
            .filter_map(move |update| fix(update, &input.rules_after))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn fix(update: &[usize], rules_after: &HashMap<usize, HashSet<usize>>) -> Option<Vec<usize>> {
        let mut out = Vec::from(update);
        let l = out.len();
        if update_valid(&out, rules_after) {
            #[cfg(test)]
            println!("Update already valid! {out:?}");
            return None;
        }
        while !update_valid(&out, rules_after) {
            for i in 0..l - 1 {
                let mut update_idx: Option<usize> = None;
                let page = out[i];
                let Some(rules) = rules_after.get(&page) else {
                    #[cfg(test)]
                    println!("No rules found for {page}");
                    continue;
                };
                #[cfg(test)]
                println!("Checking for swaps for {page} at {i}");

                for j in i + 1..l {
                    let pj = out[j];
                    #[cfg(test)]
                    println!("  Checking for swap against {pj} at j");
                    if rules.contains(&pj) {
                        #[cfg(test)]
                        println!(
                            "    {} cannot have {} after, setting swap to {}",
                            page, pj, j
                        );
                        update_idx = Some(j);
                    }
                }
                if let Some(update_idx) = update_idx {
                    #[cfg(test)]
                    println!("  Moving {page} at {i} to {update_idx}, before: {out:?}");
                    out.remove(i);
                    out.insert(update_idx, page);
                    #[cfg(test)]
                    println!("  After {out:?}")
                }
            }
        }

        Some(out)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day5.txt");
            let input = parse_input(&input);
            println!("{:?}", input.rules_after);

            assert_eq!(
                fix(&[75, 97, 47, 61, 53], &input.rules_after).unwrap(),
                vec![97, 75, 47, 61, 53]
            );
            assert_eq!(
                fix(&[61, 13, 29], &input.rules_after).unwrap(),
                vec![61, 29, 13]
            );
            assert_eq!(
                fix(&[97, 13, 75, 29, 47], &input.rules_after).unwrap(),
                vec![97, 75, 47, 29, 13]
            );

            assert_eq!(calculate(&input), 123);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();
    let input = parse_input(&input);

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
