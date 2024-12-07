pub mod aoc;
pub mod quant;

pub struct IntoArrayChunks<I, const N: usize>
where
    I: Iterator,
{
    iterator: I,
}

impl<I: Iterator, const N: usize> Iterator for IntoArrayChunks<I, N> {
    type Item = [I::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut out: Vec<_> = Vec::with_capacity(N);

        for _ in 0..N {
            if let Some(item) = self.iterator.next() {
                out.push(item);
            } else {
                return None;
            }
        }

        out.try_into().ok()
    }
}

pub fn arr_chunks<const N: usize, I>(iterator: I) -> impl Iterator<Item = [I::Item; N]>
where
    I: Iterator,
{
    IntoArrayChunks { iterator }
}

#[cfg(test)]
mod test {
    use super::arr_chunks;

    #[test]
    fn test_chunks() {
        let v = vec![0, 1, 2, 3, 4];
        let mut cs = arr_chunks(v.iter());
        assert_eq!(cs.next(), Some([&0, &1]));
        assert_eq!(cs.next(), Some([&2, &3]));
        assert_eq!(cs.next(), None);
    }
}
