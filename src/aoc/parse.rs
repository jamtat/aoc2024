use nom;

pub fn parse_number<T>(s: &str) -> nom::IResult<&str, T>
where
    T: std::str::FromStr,
{
    nom::combinator::map_res(
        nom::bytes::complete::take_while1(|c: char| c.is_ascii_digit()),
        |s: &str| s.parse::<T>(),
    )(s)
}
