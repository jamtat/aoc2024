use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut, Index, IndexMut},
    str::FromStr,
};

use aoc2024::aoc;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
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

    pub fn pack_whole_files(&mut self) {
        let mut i = 0;

        while i < self.len() {
            let free_size = match self[i] {
                Entry::Free(size) if size > 0 => size,
                _ => {
                    i += 1;
                    continue;
                }
            };
            #[cfg(test)]
            {
                println!("{}", self);
                println!("Trying to replace free size={} at index {}", free_size, i);
            }
            for j in (i + 1..self.len()).rev() {
                match self[j] {
                    Entry::File { id: _, size } if 0 < size && size <= free_size => {
                        self[i].set_size(free_size - size);
                        let to_insert = self[j];
                        self[j] = Entry::free(size);
                        self.0.insert(i, to_insert);
                        break;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }

    pub fn _compact_representation(&self) -> EntryDisk {
        // Compress all the consecutive free blocks, and remove entries of zero size
        let mut out = EntryDisk::default();
        let mut last_entry: Option<Entry> = None;

        for &entry in &self.0 {
            if entry.size() == 0 {
                continue;
            }

            if let Some(last) = last_entry {
                match (last, entry) {
                    (
                        Entry::File {
                            id: last_id,
                            size: last_size,
                        },
                        Entry::File { id, size },
                    ) => {
                        if last_id == id {
                            last_entry = Some(Entry::file(id, size + last_size))
                        } else {
                            out.0.push(last);
                            last_entry = Some(entry);
                        }
                    }
                    (Entry::File { .. }, Entry::Free(_)) | (Entry::Free(_), Entry::File { .. }) => {
                        out.0.push(last);
                        last_entry = Some(entry);
                    }
                    (Entry::Free(s1), Entry::Free(s2)) => last_entry = Some(Entry::free(s1 + s2)),
                }
            } else {
                last_entry = Some(entry)
            }
        }

        if let Some(last_entry) = last_entry {
            out.0.push(last_entry);
        }

        out
    }
}

impl Display for EntryDisk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in &self.0 {
            write!(f, "{} ", entry)?;
        }
        Ok(())
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

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entry::File { id, size } => {
                for _ in 0..*size {
                    write!(f, "{}", id)?;
                }
            }
            Entry::Free(size) => {
                for _ in 0..*size {
                    f.write_char('.')?;
                }
            }
        }
        Ok(())
    }
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
        let mut disk: EntryDisk = input.parse::<EntryDisk>().unwrap();
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
    println!("Part 2: {}", part2::calculate(&input));
}
