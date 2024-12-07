use aoc2024::aoc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

impl Instruction {
    pub fn value(&self) -> usize {
        match &self {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        }
    }
}

mod parse {
    use nom::bytes::complete::{tag, take_while_m_n};
    use nom::character::complete::{anychar, char};
    use nom::multi::{fold_many0, many_till};
    use nom::sequence::{preceded, separated_pair, terminated};

    use super::Instruction;

    fn parse_mul_num(s: &str) -> nom::IResult<&str, usize> {
        nom::combinator::map_res(
            take_while_m_n(1, 3, |c: char| c.is_ascii_digit()),
            |s: &str| s.parse::<usize>(),
        )(s)
    }

    pub fn parse_mul(s: &str) -> nom::IResult<&str, Instruction> {
        preceded(
            tag("mul("),
            terminated(
                separated_pair(parse_mul_num, char(','), parse_mul_num),
                char(')'),
            ),
        )(s)
        .map(|(s, (a, b))| (s, Instruction::Mul(a, b)))
        // tuple((
        //     tag("mul("),
        //     parse_mul_num,
        //     char(','),
        //     parse_mul_num,
        //     char(')'),
        // ))(s)
        // .map(|(s, (_, a, _, b, _))| (s, Mul(a, b)))
    }

    pub fn parse_instructions(s: &str) -> nom::IResult<&str, Vec<Instruction>> {
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

    fn parse_input(s: &str) -> Vec<Instruction> {
        parse::parse_instructions(s).unwrap().1
    }

    pub fn calculate(s: &str) -> usize {
        parse_input(s).iter().map(Instruction::value).sum()
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
        assert_eq!(
            parse::parse_mul("mul(12,240)"),
            Ok(("", Instruction::Mul(12, 240)))
        );
        assert!(parse::parse_mul("mul (1,1)").is_err());
    }
}
