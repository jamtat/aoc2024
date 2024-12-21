pub fn parse_number<T>(s: &str) -> nom::IResult<&str, T>
where
    T: std::str::FromStr,
{
    nom::combinator::map_res(
        nom::combinator::recognize(nom::sequence::preceded(
            nom::combinator::opt(nom::character::complete::char('-')),
            nom::character::complete::digit1,
        )),
        str::parse::<T>,
    )(s)
}

#[cfg(test)]
mod test {
    use crate::aoc::parse::parse_number;

    #[test]
    fn test_parse() {
        assert_eq!(parse_number::<usize>("32").unwrap().1, 32usize);

        assert_eq!(parse_number::<isize>("-64").unwrap().1, -64isize);
    }
}
