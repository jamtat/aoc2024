use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut, Index, IndexMut},
    str::FromStr,
};

use aoc2024::aoc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct EntryDisk(Vec<Entry>);

impl EntryDisk {
    pub fn checksum(&self) -> usize {
        let mut total = 0;
        let mut offset = 0;

        for entry in &self.0 {
            total += match *entry {
                Entry::File { id, size } => (offset..(offset + size)).sum::<usize>() * id,
                Entry::Free(_) => 0,
            };
            offset += entry.size();
        }

        total
    }

    pub fn is_free(&self, idx: usize) -> bool {
        self[idx].is_free()
    }

    pub fn pack_whole_files(&mut self) -> EntryDisk {
        let mut out = vec![];

        let mut i = 0;

        for i in 0..self.len() {
            let i_entry = &self[i];
            if !self.is_free(i) {
                out.push(i_entry.clone());
            }
            let free_size = self[i].size();
            for j in (i..self.len()).rev() {
                let entry = &self[j];
                if !entry.is_free() || entry.size() > free_size {
                    continue;
                }
                entry.set_size(0);
            }
        }
        EntryDisk(out)
    }
}

impl FromStr for EntryDisk {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.bytes()
                .enumerate()
                .filter_map(|(i, c)| {
                    let size: usize = (c - b'0').into();
                    (size > 0).then_some(if i % 2 == 0 {
                        Entry::file(i / 2, size)
                    } else {
                        Entry::free(size)
                    })
                })
                .collect(),
        ))
    }
}

impl Deref for EntryDisk {
    type Target = [Entry];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for EntryDisk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Index<usize> for EntryDisk {
    type Output = Entry;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for EntryDisk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Entry {
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

type Id = u32;
static FREE: Id = Id::MAX;

#[derive(Debug, Clone, PartialEq, Eq)]
struct VecDisk(Vec<Id>);

impl VecDisk {
    pub fn checksum(&self) -> usize {
        self.iter()
            .enumerate()
            .map(|(i, &id)| if id == FREE { 0 } else { i * (id as usize) })
            .sum()
    }

    fn is_free(&self, idx: usize) -> bool {
        self[idx] == FREE
    }

    pub fn pack(&mut self) {
        if self.len() == 0 {
            return;
        }
        let mut i = 0;
        let mut j = self.len() - 1;

        while i < self.len() && j > 0 && j > i {
            // Move i from the left until we get a free spot
            if !self.is_free(i) {
                i += 1;
                continue;
            }
            // Move j from the right until we get a non-free spot
            if self.is_free(j) {
                j -= 1;
                continue;
            }
            self[i] = self[j];
            self[j] = FREE;
        }
    }
}

impl FromStr for VecDisk {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.bytes()
                .enumerate()
                .flat_map(|(i, c)| {
                    let i: u32 = i.try_into().expect("Id range too large for Id size");
                    let size: usize = (c - b'0').into();
                    let id = if i % 2 == 0 { i / 2 } else { FREE };
                    std::iter::repeat(id).take(size)
                })
                .collect(),
        ))
    }
}

impl Display for VecDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_id = 0;
        for &id in self.0.iter() {
            if id != last_id {
                f.write_char(' ')?;
            }
            if id == FREE {
                f.write_char('.')?
            } else {
                write!(f, "{}", id)?
            }
            last_id = id;
        }
        Ok(())
    }
}

impl Deref for VecDisk {
    type Target = [Id];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VecDisk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for VecDisk {
    type Output = Id;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for VecDisk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_entry_disk() {
        assert_eq!(
            "90909".parse::<EntryDisk>().unwrap(),
            EntryDisk(vec![
                Entry::file(0, 9),
                Entry::file(1, 9),
                Entry::file(2, 9)
            ])
        )
    }

    #[test]
    fn test_checksum_entry_disk() {
        let disk = EntryDisk(vec![
            Entry::file(0, 2),
            Entry::file(9, 2),
            Entry::file(8, 1),
        ]);

        assert_eq!(disk.checksum(), 77);
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let mut disk: VecDisk = input.parse().unwrap();
        disk.pack();
        disk.checksum()
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

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let mut disk: EntryDisk = input.parse().unwrap();
        disk.pack_whole_files();
        disk.checksum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day9.txt");
            assert_eq!(calculate(&input), 2858);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    // println!("Part 2: {}", part2::calculate(&input));
}
