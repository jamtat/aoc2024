use aoc2024::aoc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Mul(usize, usize);

impl Mul {
    pub fn get(&self) -> usize {
        self.0 * self.1
    }
}

mod parse {
    use nom::bytes::complete::{tag, take_while_m_n};
    use nom::character::complete::{anychar, char};
    use nom::multi::{fold_many0, many_till};
    use nom::sequence::{preceded, separated_pair, terminated};

    use super::Mul;

    fn parse_mul_num(s: &str) -> nom::IResult<&str, usize> {
        nom::combinator::map_res(
            take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
            |s: &str| s.parse::<usize>(),
        )(s)
    }

    pub fn parse_mul(s: &str) -> nom::IResult<&str, Mul> {
        preceded(
            tag("mul("),
            terminated(
                separated_pair(parse_mul_num, char(','), parse_mul_num),
                char(')'),
            ),
        )(s)
        .map(|(s, (a, b))| (s, Mul(a, b)))
        // tuple((
        //     tag("mul("),
        //     parse_mul_num,
        //     char(','),
        //     parse_mul_num,
        //     char(')'),
        // ))(s)
        // .map(|(s, (_, a, _, b, _))| (s, Mul(a, b)))
    }

    pub fn parse_muls(s: &str) -> nom::IResult<&str, Vec<Mul>> {
        fold_many0(
            many_till(anychar, parse_mul),
            Vec::new,
            |mut acc: Vec<_>, (_, mul)| {
                acc.push(mul);
                acc
            },
        )(s)
    }
}

mod part1 {
    use super::*;

    fn parse_input(s: &str) -> Vec<Mul> {
        parse::parse_muls(s).unwrap().1
    }

    pub fn calculate(s: &str) -> usize {
        parse_input(s).iter().map(Mul::get).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day3.txt");

            assert_eq!(calculate(&input), 161);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();
    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_mul() {
        assert_eq!(parse::parse_mul("mul(12,240)"), Ok(("", Mul(12, 240))));
        assert!(parse::parse_mul("mul (1,1)").is_err());
    }
}
