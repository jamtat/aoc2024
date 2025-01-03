use aoc2024::aoc;

#[allow(dead_code)]
pub mod computer {
    use std::{
        fmt::{Display, Write},
        ops::Not,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct OpCode(pub u8);

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

    pub struct Operand(OpCode);

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

    impl Display for CommboOperand {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[repr(u8)]
    #[derive(Debug)]
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

    impl Display for Instruction {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
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

        pub fn read(&self) -> Option<(Instruction, Operand)> {
            (self.ip >= self.program.len()).not().then(|| {
                (
                    self.program[self.ip].into(),
                    Operand(self.program[self.ip + 1]),
                )
            })
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
            let Some((instruction, operand)) = self.read() else {
                return InstructionResult::Halt;
            };

            self.ip += 2;

            match instruction {
                Instruction::ADV => {
                    self.a >>= operand.combo_value(self);
                }
                Instruction::BXL => {
                    self.b ^= operand.literal();
                }
                Instruction::BST => {
                    self.b = operand.combo_value(self) & 7;
                }
                Instruction::JNZ => {
                    if self.a != 0 {
                        self.ip = operand.literal().try_into().unwrap();
                    }
                }
                Instruction::BXC => {
                    self.b ^= self.c;
                }
                Instruction::OUT => {
                    return InstructionResult::Output(
                        (operand.combo_value(self) & 7).try_into().unwrap(),
                    );
                }
                Instruction::BDV => {
                    self.b = self.a >> operand.combo_value(self);
                }
                Instruction::CDV => {
                    self.c = self.a >> operand.combo_value(self);
                }
            }
            InstructionResult::Nothing
        }

        pub fn iter_output(&mut self) -> impl Iterator<Item = u8> + '_ {
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

        // println!("Before:\n{computer}");

        // let mut output: Vec<String> = vec![];

        // let mut i = 0;
        // loop {
        //     i += 1;
        //     let Some((instruction, operand)) = computer.read() else {
        //         break;
        //     };
        //     let combo = operand.combo();
        //     let literal = operand.literal();
        //     println!("---\nAfter Step {i}:");
        //     println!(" Instruction: {instruction}");
        //     println!(" Combo: {combo}");
        //     println!(" Literal: {literal}");

        //     let result = computer.step();

        //     match result {
        //         computer::InstructionResult::Output(n) => {
        //             println!("!!Output: {n}");
        //             output.push(n.to_string())
        //         }
        //         computer::InstructionResult::Nothing => {}
        //         computer::InstructionResult::Halt => break,
        //     }
        //     println!("{computer}");
        //     println!("Total output: {}", output.join(","));
        // }

        computer
            .iter_output()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(",")
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
        fn test_input() {
            let input = aoc::cli::input_string("day17.txt");
            assert_eq!(calculate(&input), "1,2,3,1,3,2,5,3,1");
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

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> isize {
        let computer = computer::parse(input).expect("Could not parse computer");
        let target = computer.program.iter().map(|op| op.0).collect::<Vec<_>>();
        // println!("Target: {:?}", target);

        /*
        2,4, 1,5, 7,5, 1,6, 0,3, 4,3, 5,5, 3,0

        2,4 | bst a     | b = a & 7      # take last 3 bits of a into b
        1,5 | bxl 5     | b = b ^ 5      # manipulate first 3 bits of b
        7,5 | cdv b     | c = a >> b     # shift a up to 7 bits along
        1,6 | bxl 6     | b = b ^ 6      # manipulate first 3 bits of b again
        0,3 | adv 3     | a = a >> 3     # drop bottom 3 bits of a
        4,3 | bxc _     | b = b ^ c      # using last 3 bits of a and up the next 7 of a (i.e. last 10 bits) manipulate b again
        5,5 | out b     |                # output is based on up to the last 10 bits of a
        3,0 | jmp 0
        */

        // Find all the combos of the first 10 bits that produce the first output
        let saved: Vec<isize> = (0..2 << 9)
            .filter(|a| {
                let mut computer = computer.clone();
                computer.a = *a;
                let output = computer.iter_output().next().unwrap();
                output == target[0]
            })
            .collect();

        // Now go up by three bits at a time, finding all the next 3 bits that produce the next
        // value in the output
        target
            .iter()
            .enumerate()
            .skip(1)
            .fold(saved, |saved, (pos, &target)| {
                let mut next = vec![];
                for consider in saved {
                    for first3bits in 0..1 << 3 {
                        let a = (first3bits << (7 + 3 * pos)) | consider;
                        let mut computer = computer.clone();
                        computer.a = a;
                        let output = computer.iter_output().nth(pos).unwrap_or(u8::MAX);
                        if output == target {
                            next.push(a);
                        }
                    }
                }
                next
            })
            .iter()
            .min()
            .cloned()
            .unwrap()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day17_2.txt");
            assert_eq!(calculate(&input), 117440);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
