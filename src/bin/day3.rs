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
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while_m_n};
    use nom::character::complete::{anychar, char};
    use nom::combinator::map;
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

    pub fn parse_do(s: &str) -> nom::IResult<&str, Instruction> {
        map(tag("do()"), |_| Instruction::Do)(s)
    }

    pub fn parse_dont(s: &str) -> nom::IResult<&str, Instruction> {
        map(tag("don't()"), |_| Instruction::Dont)(s)
    }

    pub fn parse_instruction(s: &str) -> nom::IResult<&str, Instruction> {
        alt((parse_mul, parse_do, parse_dont))(s)
    }

    pub fn parse_instructions(s: &str) -> nom::IResult<&str, Vec<Instruction>> {
        fold_many0(
            many_till(anychar, parse_instruction),
            Vec::new,
            |mut acc: Vec<_>, (_, mul)| {
                acc.push(mul);
                acc
            },
        )(s)
    }
}

fn parse_input(s: &str) -> Vec<Instruction> {
    parse::parse_instructions(s).unwrap().1
}

mod part1 {
    use super::*;

    pub fn calculate(s: &str) -> usize {
        parse_input(s).iter().map(Instruction::value).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day3_1.txt");

            assert_eq!(calculate(&input), 161);
        }
    }
}

mod part2 {
    use super::*;

    pub fn calculate(s: &str) -> usize {
        parse_input(s)
            .into_iter()
            .fold((0, true), |(total, enabled), instruction| {
                let enabled = match instruction {
                    Instruction::Mul(_, _) => enabled,
                    Instruction::Do => true,
                    Instruction::Dont => false,
                };

                (
                    total + if enabled { instruction.value() } else { 0 },
                    enabled,
                )
            })
            .0
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day3_2.txt");

            assert_eq!(calculate(&input), 48);
        }

        #[test]
        fn test_parse() {
            assert_eq!(
                parse_input(&aoc::example::example_string("day3_2.txt")),
                vec![
                    Instruction::Mul(2, 4),
                    Instruction::Dont,
                    Instruction::Mul(5, 5),
                    Instruction::Mul(11, 8),
                    Instruction::Do,
                    Instruction::Mul(8, 5),
                ]
            );
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();
    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
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
