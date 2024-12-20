use std::fmt::Display;

use aoc2024::aoc;

#[derive(Clone, Copy)]
struct Button {
    x: isize,
    y: isize,
}

struct Game {
    a: Button,
    b: Button,
    prize_x: isize,
    prize_y: isize,
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

    fn parse_prize(s: &str) -> nom::IResult<&str, (isize, isize)> {
        let (s, _) = tag("Prize: X=")(s)?;
        let (s, x) = parse_number(s)?;
        let (s, _) = tag(", Y=")(s)?;
        let (s, y) = parse_number(s)?;

        Ok((s, (x, y)))
    }
}

static A_COST: isize = 3;
static B_COST: isize = 1;

mod part1 {
    use super::*;

    fn min_tokens(game: &Game) -> Option<isize> {
        for a in 0..=100 {
            let x = game.a.x * a;
            let y = game.a.y * a;
            for b in 0..=100 {
                let x = x + game.b.x * b;
                let y = y + game.b.y * b;

                if x == game.prize_x && y == game.prize_y {
                    return Some(a * A_COST + b * B_COST);
                }
            }
        }
        None
    }

    pub fn calculate(input: &str) -> isize {
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

mod part2 {
    use super::*;

    static OFFSET: isize = 10000000000000;

    fn solve(
        Game {
            a: Button { x: ax, y: ay },
            b: Button { x: bx, y: by },
            prize_x: px,
            prize_y: py,
        }: &Game,
    ) -> Option<usize> {
        // Shamelessly stolen from https://www.youtube.com/watch?v=-5J-DAsWuJc
        // because I didn't remember systems of linear equations from school
        let numerator = px * by - py * bx;
        let denominator = ax * by - ay * bx;
        if numerator % denominator != 0 {
            return None;
        }
        let a_presses = numerator / denominator;

        let numerator = px - ax * a_presses;
        if numerator % bx != 0 {
            return None;
        }
        let b_presses = numerator / bx;
        Some((a_presses * A_COST + b_presses * B_COST) as usize)
    }

    pub fn calculate(input: &str) -> usize {
        parse::parse_input(input)
            .iter()
            .map(|game| Game {
                a: game.a,
                b: game.b,
                prize_x: game.prize_x + OFFSET,
                prize_y: game.prize_y + OFFSET,
            })
            .filter_map(|game| solve(&game))
            .sum()
    }

    // No example given!!
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
