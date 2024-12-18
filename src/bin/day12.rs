use std::collections::{HashSet, VecDeque};

use aoc2024::{
    aoc::{
        self,
        grid::{Direction, Grid, Point},
    },
    point2d::{Point2D, Point2Disize},
};

type Garden = Grid<Vec<char>>;

fn to_point2d(point: &Point) -> Point2Disize {
    Point2Disize::new(point.x as isize, point.y as isize)
}

fn neighbours(point2d: &Point2Disize) -> [Point2Disize; 4] {
    [
        *point2d + Point2Disize::new(-1, 0),
        *point2d + Point2Disize::new(1, 0),
        *point2d + Point2Disize::new(0, -1),
        *point2d + Point2Disize::new(0, 1),
    ]
}

#[derive(Debug, Clone)]
struct Region {
    plant: char,
    points: HashSet<Point>,
}

impl Region {
    pub fn new(plant: char) -> Self {
        Region {
            plant,
            points: Default::default(),
        }
    }

    pub fn add_point(&mut self, point: &Point) {
        self.points.insert(*point);
    }

    pub fn area(&self) -> usize {
        self.points.len()
    }

    pub fn perimeter(&self) -> usize {
        let points2d: HashSet<_> = self.points.iter().map(to_point2d).collect();

        points2d
            .iter()
            .flat_map(neighbours)
            .filter(|p| !points2d.contains(p))
            .count()
    }

    pub fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

fn regions(garden: &Garden) -> Vec<Region> {
    let mut collected: HashSet<Point> = Default::default();
    let mut regions = vec![];

    for cell in garden.iter() {
        let p = cell.point();
        let plant = *cell.value();

        if collected.contains(&p) {
            continue;
        }

        let mut queue = VecDeque::new();
        queue.push_back(cell);

        let mut region = Region::new(plant);
        region.add_point(&p);

        while let Some(cell) = queue.pop_front() {
            for direction in Direction::all() {
                let Some(cell) = cell.go(direction) else {
                    continue;
                };
                let p = cell.point();
                if collected.contains(&p) || *cell.value() != region.plant {
                    continue;
                }
                region.add_point(&p);
                collected.insert(p);
                queue.push_back(cell);
            }
        }
        regions.push(region);
    }
    regions
}

mod part1 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let garden: Garden = input.parse().unwrap();
        let regions = regions(&garden);

        // for region in &regions {
        //     println!(
        //         "A region of {} plants with price {} * {} = {}",
        //         region.plant,
        //         region.area(),
        //         region.perimeter(),
        //         region.price()
        //     );
        // }

        regions.iter().map(Region::price).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day12.txt");
            assert_eq!(calculate(&input), 1930);
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
            let input = aoc::example::example_string("day12.txt");
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
