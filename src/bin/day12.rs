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
        *point2d + (-1, 0),
        *point2d + (1, 0),
        *point2d + (0, -1),
        *point2d + (0, 1),
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

    fn points2d(&self) -> impl Iterator<Item = Point2Disize> + '_ {
        self.points.iter().map(to_point2d)
    }

    pub fn area(&self) -> usize {
        self.points.len()
    }

    pub fn perimeter(&self) -> usize {
        let points2d: HashSet<_> = self.points2d().collect();

        points2d
            .iter()
            .flat_map(neighbours)
            .filter(|p| !points2d.contains(p))
            .count()
    }

    pub fn sides(&self) -> usize {
        let points2d: HashSet<_> = self.points2d().collect();
        let mut vert: Vec<isize> = vec![];
        let mut horz: Vec<isize> = vec![];

        for &p in points2d.iter() {
            if !points2d.contains(&(p + (-1, 0))) {
                horz.push(p.x);
            }
            if !points2d.contains(&(p + (1, 0))) {
                horz.push(p.x + 1);
            }

            if !points2d.contains(&(p + (0, -1))) {
                vert.push(p.y);
            }
            if !points2d.contains(&(p + (0, 1))) {
                vert.push(p.y + 1);
            }
        }

        let edge_points = points2d.iter().map(|&p| {
            let mut vert: Vec<isize> = vec![];
            let mut horz: Vec<isize> = vec![];

            if !points2d.contains(&(p + (-1, 0))) {
                vert.push(value);
            }
            if !points2d.contains(&(p + (1, 0))) {
                edges.push(p + (1, 0));
            }

            if !points2d.contains(&(p + (0, -1))) {
                edges.push(p);
            }
            if !points2d.contains(&(p + (0, 1))) {
                edges.push(p + (1, 0));
            }

            (vert, horz)
        });

        // let perimeter_points: HashSet<_> = points2d
        //     .iter()
        //     .flat_map(neighbours)
        //     .filter(|p| !points2d.contains(p))
        //     .collect();

        // let edge_points = self.
        // let start = perimeter_points.iter().next().unwrap();

        0
    }
}

struct Side {
    horizontal: bool,
    min: usize,
    max: usize,
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

        regions.iter().map(|r| r.area() * r.perimeter()).sum()
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

mod part2 {
    use super::*;

    pub fn calculate(input: &str) -> usize {
        let garden: Garden = input.parse().unwrap();
        let regions = regions(&garden);
        regions.iter().map(|r| r.area() * r.sides()).sum()
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_example() {
            let input = aoc::example::example_string("day12.txt");
            assert_eq!(calculate(&input), 1206);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
