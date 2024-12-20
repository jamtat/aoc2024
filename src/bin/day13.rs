use std::fmt::Display;

use aoc2024::aoc;

struct Button {
    x: usize,
    y: usize,
}

struct Game {
    a: Button,
    b: Button,
    prize_x: usize,
    prize_y: usize,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Button A: X+{}, Y+{}", self.a.x, self.a.y)?;
        writeln!(f, "Button B: X+{}, Y+{}", self.b.x, self.b.y)?;
        write!(f, "Prize: X={}, Y={}", self.prize_x, self.prize_y)
    }
}

mod parse {
    use aoc2024::aoc::parse::parse_number;
    use nom::{
        bytes::complete::tag, character::complete::char, multi::separated_list0, sequence::tuple,
    };

    use super::{Button, Game};

    pub fn parse_input(s: &str) -> Vec<Game> {
        separated_list0(tag("\n\n"), parse_game)(s).unwrap().1
    }

    fn parse_game(s: &str) -> nom::IResult<&str, Game> {
        let (s, (a, _, b, _, (prize_x, prize_y))) = tuple((
            parse_button,
            char('\n'),
            parse_button,
            char('\n'),
            parse_prize,
        ))(s)?;

        Ok((
            s,
            Game {
                a,
                b,
                prize_x,
                prize_y,
            },
        ))
    }

    fn parse_button(s: &str) -> nom::IResult<&str, Button> {
        let (s, _) = nom::branch::alt((tag("Button A: X+"), tag("Button B: X+")))(s)?;
        let (s, x) = parse_number(s)?;
        let (s, _) = tag(", Y+")(s)?;
        let (s, y) = parse_number(s)?;

        Ok((s, Button { x, y }))
    }

    fn parse_prize(s: &str) -> nom::IResult<&str, (usize, usize)> {
        let (s, _) = tag("Prize: X=")(s)?;
        let (s, x) = parse_number(s)?;
        let (s, _) = tag(", Y=")(s)?;
        let (s, y) = parse_number(s)?;

        Ok((s, (x, y)))
    }
}

mod part1 {
    use super::*;

    static A_COST: usize = 3;
    static B_COST: usize = 1;

    fn min_tokens(game: &Game) -> Option<usize> {
        let mut min = None;
        for a in 0..=100 {
            let x = game.a.x * a;
            let y = game.a.y * a;
            for b in 0..=100 {
                let x = x + game.b.x * b;
                let y = y + game.b.y * b;

                if x == game.prize_x && y == game.prize_y {
                    min = Some(a * A_COST + b * B_COST)
                }
            }
        }
        min
    }

    pub fn calculate(input: &str) -> usize {
        let games = parse::parse_input(input);
        #[cfg(test)]
        for game in &games {
            println!("{}\n\n", game);
        }

        games.iter().filter_map(min_tokens).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day13.txt");
            assert_eq!(calculate(&input), 480);
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
            let input = aoc::example::example_string("day13.txt");
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
