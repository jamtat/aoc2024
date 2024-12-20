use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
    fmt::Display,
};

use aoc2024::{
    aoc::{
        self,
        grid::{Direction, Grid, Point},
    },
    point2d::Point2Disize,
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

#[derive(Debug, Clone, Copy)]
struct Edge {
    direction: Direction,
    x: isize,
    y: isize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.direction.cmp(&other.direction) {
            Ordering::Equal => {}
            ord => return ord,
        }
        match self.direction {
            Direction::Up | Direction::Down => {
                match self.y.cmp(&other.y) {
                    Ordering::Equal => {}
                    ord => return ord,
                }
                self.x.cmp(&other.x)
            }
            Direction::Left | Direction::Right => {
                match self.x.cmp(&other.x) {
                    Ordering::Equal => {}
                    ord => return ord,
                }
                self.y.cmp(&other.y)
            }
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction && self.x == other.x && self.y == other.y
    }
}

impl Eq for Edge {}

impl Edge {
    pub fn new(direction: Direction, x: isize, y: isize) -> Self {
        Self { direction, x, y }
    }

    pub fn connects(&self, &Self { direction, x, y }: &Self) -> bool {
        self.direction == direction
            && match self.direction {
                Direction::Left | Direction::Right => {
                    self.x == x && self.y >= y - 1 && self.y <= y + 1
                }
                Direction::Up | Direction::Down => {
                    self.y == y && self.x >= x - 1 && self.x <= x + 1
                }
            }
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.direction, self.x, self.y)
    }
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

        let mut edges: Vec<_> = vec![];

        for &p in &points2d {
            if !points2d.contains(&(p - (1, 0))) {
                edges.push(Edge::new(Direction::Left, p.x, p.y));
            }
            if !points2d.contains(&(p + (1, 0))) {
                edges.push(Edge::new(Direction::Right, p.x + 1, p.y));
            }

            if !points2d.contains(&(p - (0, 1))) {
                edges.push(Edge::new(Direction::Up, p.x, p.y));
            }
            if !points2d.contains(&(p + (0, 1))) {
                edges.push(Edge::new(Direction::Down, p.x, p.y + 1));
            }
        }

        edges.sort();
        let mut count = 0;
        let mut last_edge: Option<Edge> = None;

        for edge in edges {
            let Some(le) = last_edge else {
                last_edge = Some(edge);
                count = 1;
                continue;
            };

            if !edge.connects(&le) {
                count += 1;
            }

            last_edge = Some(edge);
        }

        count
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
        #[cfg(test)]
        {
            println!();
            for r in &regions {
                println!(
                    "Region {} area of {} and {} sides",
                    r.plant,
                    r.area(),
                    r.sides()
                );
            }
        }
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

        #[test]
        fn test_example2() {
            let input = aoc::example::example_string("day12_2.txt");
            assert_eq!(calculate(&input), 368);
        }

        #[test]
        fn test_example3() {
            let input = aoc::example::example_string("day12_3.txt");
            assert_eq!(calculate(&input), 236);
        }
    }
}

fn main() {
    let cli = aoc::cli::parse();

    let input = cli.input_string();

    println!("Part 1: {}", part1::calculate(&input));
    println!("Part 2: {}", part2::calculate(&input));
}
