use aoc2024::aoc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Entry {
    File { id: usize, size: usize },
    Free(usize),
}

impl Entry {
    pub fn file(id: usize, size: usize) -> Self {
        Self::File { id, size }
    }
    pub fn free(size: usize) -> Self {
        Self::Free(size)
    }
    pub fn size(&self) -> usize {
        match self {
            Entry::File { id: _, size } => *size,
            Entry::Free(size) => *size,
        }
    }
    pub fn is_free(&self) -> bool {
        match self {
            Entry::File { .. } => false,
            Entry::Free(size) => *size > 0,
        }
    }
    pub fn set_size(&mut self, size: usize) {
        match self {
            Entry::File { id: _, size: s } => *s = size,
            Entry::Free(s) => *s = size,
        }
    }
}

fn checksum(disk: &[Entry]) -> usize {
    let mut total = 0;
    let mut offset = 0;

    for entry in disk {
        total += match *entry {
            Entry::File { id, size } => (offset..(offset + size)).sum::<usize>() * id,
            Entry::Free(_) => 0,
        };
        offset += entry.size();
    }

    total
}

fn fill_next_gap(disk: &[Entry]) -> Vec<Entry> {
    let mut out = vec![];

    out
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .bytes()
        .enumerate()
        .flat_map(|(i, c)| {
            let size: usize = (c - b'0').into();
            (size > 0).then_some(if i % 2 == 0 {
                Entry::file(i / 2, size)
            } else {
                Entry::free(size)
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_input("90909"),
            vec![Entry::file(0, 9), Entry::file(1, 9), Entry::file(2, 9)]
        )
    }

    #[test]
    fn test_checksum() {
        let disk = &[Entry::file(0, 2), Entry::file(9, 2), Entry::file(8, 1)];

        assert_eq!(checksum(disk), 77);
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let disk = parse_input(input);

        0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day9.txt");
            assert_eq!(calculate(&input), 1928);
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
            let input = aoc::example::example_string("day9.txt");
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
