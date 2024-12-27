use aoc2024::aoc;

#[allow(dead_code)]
pub mod computer {
    use std::fmt::{Display, Write};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpCode(u8);

    #[derive(Debug, Clone)]
    pub enum OpcodeParseError<T> {
        OutOfRange(T),
    }

    impl OpCode {
        pub fn from_u8(n: u8) -> Result<Self, OpcodeParseError<u8>> {
            match n {
                0..=7 => Ok(Self(n)),
                _ => Err(OpcodeParseError::OutOfRange(n)),
            }
        }

        pub fn from_int<T>(n: T) -> Result<Self, OpcodeParseError<T>>
        where
            T: Copy + TryInto<u8>,
        {
            let as_u8: u8 = n.try_into().map_err(|_| OpcodeParseError::OutOfRange(n))?;
            Self::from_u8(as_u8).map_err(|_| OpcodeParseError::OutOfRange(n))
        }
    }

    struct Operand(OpCode);

    impl Operand {
        pub fn literal(&self) -> isize {
            self.0 .0 as isize
        }

        pub fn combo(&self) -> CommboOperand {
            self.0.into()
        }

        pub fn combo_value(&self, computer: &Computer) -> isize {
            self.combo().value(computer)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum CommboOperand {
        Literal(u8),
        A,
        B,
        C,
    }

    impl CommboOperand {
        fn value(&self, computer: &Computer) -> isize {
            match self {
                CommboOperand::Literal(n) => (*n).into(),
                CommboOperand::A => computer.a,
                CommboOperand::B => computer.b,
                CommboOperand::C => computer.c,
            }
        }
    }

    impl From<OpCode> for CommboOperand {
        fn from(OpCode(n): OpCode) -> Self {
            match n {
                0..=3 => Self::Literal(n),
                4 => Self::A,
                5 => Self::B,
                6 => Self::C,
                _ => panic!("Invalid OpCode({n})"),
            }
        }
    }

    pub enum Instruction {
        // The adv instruction (opcode 0) performs division. The numerator is the value in the A register.
        // The denominator is found by raising 2 to the power of the instruction's combo operand.
        // (So, an operand of 2 would divide A by 4 (2^2); an operand of 5 would divide A by 2^B.)
        // The result of the division operation is truncated to an integer and then written to the A register.
        ADV,

        // The bxl instruction (opcode 1) calculates the bitwise XOR of register B and the instruction's
        // literal operand, then stores the result in register B.
        BXL,

        // The bst instruction (opcode 2) calculates the value of its combo operand modulo 8 (thereby
        // keeping only its lowest 3 bits), then writes that value to the B register.
        BST,

        // The jnz instruction (opcode 3) does nothing if the A register is 0. However, if the A register
        // is not zero, it jumps by setting the instruction pointer to the value of its literal operand;
        // if this instruction jumps, the instruction pointer is not increased by 2 after this instruction.
        JNZ,

        // The bxc instruction (opcode 4) calculates the bitwise XOR of register B and register C, then
        // stores the result in register B. (For legacy reasons, this instruction reads an operand but
        // ignores it.)
        BXC,

        // The out instruction (opcode 5) calculates the value of its combo operand modulo 8, then outputs
        // that value. (If a program outputs multiple values, they are separated by commas.)
        OUT,

        // The bdv instruction (opcode 6) works exactly like the adv instruction except that the result
        // is stored in the B register. (The numerator is still read from the A register.)
        BDV,

        // The cdv instruction (opcode 7) works exactly like the adv instruction except that the result is
        // stored in the C register. (The numerator is still read from the A register.)
        CDV,
    }

    impl From<OpCode> for Instruction {
        fn from(OpCode(n): OpCode) -> Self {
            match n {
                0 => Self::ADV,
                1 => Self::BXL,
                2 => Self::BST,
                3 => Self::JNZ,
                4 => Self::BXC,
                5 => Self::OUT,
                6 => Self::BDV,
                7 => Self::CDV,
                _ => panic!("Invalid OpCode({n})"),
            }
        }
    }

    pub enum InstructionResult {
        Output(u8),
        Nothing,
        Halt,
    }

    #[derive(Debug, Default, Clone)]
    pub struct Computer {
        pub a: isize,
        pub b: isize,
        pub c: isize,

        pub program: Vec<OpCode>,
        pub ip: usize,
    }

    impl Computer {
        pub fn new(a: isize, b: isize, c: isize, program: Vec<OpCode>) -> Self {
            Self {
                a,
                b,
                c,
                program,
                ip: 0,
            }
        }

        fn read(&self) -> (Instruction, Operand) {
            (
                self.program[self.ip].into(),
                Operand(self.program[self.ip + 1]),
            )
        }

        pub fn try_set_program<I, O>(&mut self, program: I) -> Result<(), OpcodeParseError<O>>
        where
            O: Copy + TryInto<u8>,
            I: IntoIterator<Item = O>,
        {
            self.program = program
                .into_iter()
                .map(|o| OpCode::from_int(o))
                .collect::<Result<Vec<OpCode>, _>>()?;
            Ok(())
        }

        pub fn step(&mut self) -> InstructionResult {
            if self.ip >= self.program.len() {
                return InstructionResult::Halt;
            }

            let (instruction, operand) = self.read();

            self.ip += 2;

            match instruction {
                Instruction::ADV => {
                    self.a /= 2isize.pow(operand.combo_value(self).try_into().unwrap());
                }
                Instruction::BXL => {
                    self.b ^= operand.literal();
                }
                Instruction::BST => {
                    self.b = operand.combo_value(self) % 8;
                }
                Instruction::JNZ => {
                    if self.a != 0 {
                        self.ip = operand.literal().try_into().unwrap();
                        return InstructionResult::Nothing;
                    }
                }
                Instruction::BXC => {
                    self.b ^= self.c;
                }
                Instruction::OUT => {
                    return InstructionResult::Output((operand.combo_value(self) % 8) as u8);
                }
                Instruction::BDV => {
                    self.b /= 2isize.pow(operand.combo_value(self).try_into().unwrap());
                }
                Instruction::CDV => {
                    self.c /= 2isize.pow(operand.combo_value(self).try_into().unwrap());
                }
            }
            InstructionResult::Nothing
        }

        pub fn iter_output(&mut self) -> impl Iterator<Item = u8> + use<'_> {
            self.flatten()
        }

        pub fn output(&mut self) -> Vec<u8> {
            self.iter_output().collect()
        }

        pub fn run(&mut self) {
            for _ in self {}
        }
    }

    impl Display for Computer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Register A: {}", self.a)?;
            writeln!(f, "Register B: {}", self.b)?;
            writeln!(f, "Register C: {}", self.c)?;

            write!(f, "\nProgram: ")?;

            for (i, &OpCode(op)) in self.program.iter().enumerate() {
                if i != 0 {
                    f.write_char(',')?;
                }
                write!(f, "{}", op)?;
            }
            f.write_char('\n')?;
            for _ in 0..(9 + self.ip * 2) {
                f.write_char(' ')?;
            }
            f.write_char('^')
        }
    }

    impl Iterator for Computer {
        type Item = Option<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.step() {
                InstructionResult::Output(n) => Some(Some(n)),
                InstructionResult::Nothing => Some(None),
                InstructionResult::Halt => None,
            }
        }
    }

    pub fn parse(s: &str) -> Option<Computer> {
        parse::parse_computer(s).map(|(_, computer)| computer).ok()
    }

    mod parse {
        use aoc2024::aoc::parse::parse_number;
        use nom::{
            bytes::complete::tag,
            character::complete::{char, newline},
            combinator::map_res,
            multi::separated_list1,
            sequence::{preceded, terminated},
        };

        use super::{Computer, OpCode};

        pub fn parse_computer(s: &str) -> nom::IResult<&str, Computer> {
            let (s, a) = terminated(preceded(tag("Register A: "), parse_number), newline)(s)?;
            let (s, b) = terminated(preceded(tag("Register B: "), parse_number), newline)(s)?;
            let (s, c) = terminated(preceded(tag("Register C: "), parse_number), newline)(s)?;

            let (s, program) = preceded(
                tag("\nProgram: "),
                separated_list1(char(','), map_res(parse_number, OpCode::from_u8)),
            )(s)?;

            Ok((s, Computer::new(a, b, c, program)))
        }
    }
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> String {
        let mut computer = computer::parse(input).expect("Could not parse computer");

        println!("Before:\n{computer}");

        let out = computer
            .iter_output()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",");

        println!("After:\n{computer}");

        out
    }

    #[allow(clippy::field_reassign_with_default)]
    #[cfg(test)]
    mod test {
        use computer::Computer;

        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day17.txt");
            assert_eq!(calculate(&input), "4,6,3,5,6,3,5,2,1,0");
        }

        #[test]
        fn test_simple_1() {
            let mut computer = Computer::default();
            computer.c = 9;
            computer.try_set_program([2, 6]).unwrap();
            computer.run();

            assert_eq!(computer.b, 1);
        }

        #[test]
        fn test_simple_2() {
            let mut computer = Computer::default();
            computer.a = 10;
            computer.try_set_program([5, 0, 5, 1, 5, 4]).unwrap();

            assert_eq!(computer.output(), vec![0, 1, 2]);
        }

        #[test]
        fn test_simple_3() {
            let mut computer = Computer::default();
            computer.a = 2024;
            computer.try_set_program([0, 1, 5, 4, 3, 0]).unwrap();

            assert_eq!(computer.output(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
            assert_eq!(computer.a, 0);
        }

        #[test]
        fn test_simple_4() {
            let mut computer = Computer::default();
            computer.b = 29;
            computer.try_set_program([1, 7]).unwrap();
            computer.run();

            assert_eq!(computer.b, 26);
        }

        #[test]
        fn test_simple_5() {
            let mut computer = Computer::default();
            computer.b = 2024;
            computer.c = 43690;
            computer.try_set_program([4, 0]).unwrap();
            computer.run();

            assert_eq!(computer.b, 44354);
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
            let input = aoc::example::example_string("day17.txt");
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
