use aoc2024::{aoc, point2d::Point2D};

type Vec2 = Point2D<isize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pub position: Vec2,
    pub velocity: Vec2,
}

#[allow(dead_code)]
impl Robot {
    pub fn new(position: (isize, isize), velocity: (isize, isize)) -> Self {
        Self {
            position: position.into(),
            velocity: velocity.into(),
        }
    }
    pub fn run(&self, ticks: usize, width: usize, height: usize) -> Self {
        let dp = self.velocity * ticks as isize;
        let bounds = Vec2::new(width as isize, height as isize);
        Robot {
            position: (self.position + dp).rem_euclid(bounds),
            velocity: self.velocity,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Robot, Vec2};

    #[test]
    fn test_run() {
        let robot = Robot::new((2, 4), (2, -3));
        let (width, height) = (11, 7);
        assert_eq!(robot.run(1, width, height).position, Vec2::new(4, 1));
        assert_eq!(robot.run(2, width, height).position, Vec2::new(6, 5));
    }
}

mod parse {
    use aoc2024::aoc::parse::parse_number;
    use nom::{
        bytes::complete::tag,
        character::complete::{char, newline},
        combinator::map,
        multi::separated_list1,
        sequence::{preceded, separated_pair},
    };

    use crate::{Robot, Vec2};

    pub fn parse_input(s: &str) -> Vec<Robot> {
        separated_list1(newline, parse_robot)(s).unwrap().1
    }

    fn parse_robot(s: &str) -> nom::IResult<&str, Robot> {
        map(
            separated_pair(preceded(tag("p="), parse_vec2), tag(" v="), parse_vec2),
            |(position, velocity)| Robot { position, velocity },
        )(s)
    }

    fn parse_vec2(s: &str) -> nom::IResult<&str, Vec2> {
        map(
            separated_pair(parse_number, char(','), parse_number),
            Vec2::from,
        )(s)
    }

    #[cfg(test)]
    mod test {
        use crate::{
            parse::{parse_robot, parse_vec2},
            Robot, Vec2,
        };

        #[test]
        fn test_parse_vec2() {
            assert_eq!(parse_vec2("0,4").unwrap(), ("", Vec2::new(0, 4)));
            assert_eq!(parse_vec2("3,-3").unwrap(), ("", Vec2::new(3, -3)));
        }

        #[test]
        fn test_parse_robot() {
            assert_eq!(
                parse_robot("p=0,4 v=3,-3").unwrap(),
                (
                    "",
                    Robot {
                        position: Vec2::new(0, 4),
                        velocity: Vec2::new(3, -3)
                    }
                )
            )
        }
    }
}

mod part1 {
    use std::collections::HashMap;

    use super::*;

    fn safety_factor(robots: &[Robot], ticks: usize, width: usize, height: usize) -> usize {
        let robots = robots
            .iter()
            .map(|r| r.run(ticks, width, height))
            .collect::<Vec<_>>();
        let mut quadrants: HashMap<(bool, bool), usize> = HashMap::new();
        quadrants.insert((false, false), 0);
        quadrants.insert((false, true), 0);
        quadrants.insert((true, false), 0);
        quadrants.insert((true, true), 0);

        let wdiv = (width / 2) as isize;
        let hdiv = (height / 2) as isize;
        // println!("{:?}", robots);

        for r in &robots {
            let x = r.position.x;
            let y = r.position.y;
            if x == wdiv || y == hdiv {
                continue;
            }

            *quadrants.entry((x < wdiv, y < hdiv)).or_default() += 1;
        }

        quadrants.values().product()
    }

    pub fn calculate(input: &str) -> usize {
        let robots = parse::parse_input(input);
        safety_factor(&robots, 100, 101, 103)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day14.txt");
            let robots = parse::parse_input(&input);
            assert_eq!(safety_factor(&robots, 100, 11, 7), 12);
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
            let input = aoc::example::example_string("day14.txt");
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
