use std::{
    fmt::{Display, Write},
    ops::{Deref, Index},
};

pub struct Grid<T: Index<usize>> {
    width: usize,
    height: usize,
    items: T,
}

impl<T: Index<usize>> Grid<T> {
    pub fn new(width: usize, height: usize, items: T) -> Grid<T> {
        Self {
            width,
            height,
            items,
        }
    }

    // Used for indexing as well
    pub fn cell_at(&self, x: usize, y: usize) -> Option<GridCell<'_, T>> {
        (x < self.width && y < self.height).then_some(GridCell { grid: self, x, y })
    }

    pub fn value_at_unchecked(&self, x: usize, y: usize) -> &T::Output {
        &self.items[y * self.width + x]
    }

    pub fn iter(&self) -> impl Iterator<Item = GridCell<'_, T>> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| GridCell { grid: self, x, y }))
    }
}

impl<T> Display for Grid<T>
where
    T: Index<usize>,
    T::Output: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                f.write_char('\n')?;
            }
            for x in 0..self.width {
                write!(f, "{}", self.value_at_unchecked(x, y))?;
            }
        }
        Ok(())
    }
}

pub struct GridCell<'a, T: Index<usize>> {
    grid: &'a Grid<T>,
    pub x: usize,
    pub y: usize,
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn all() -> &'static [Direction] {
        &[
            Direction::UP,
            Direction::DOWN,
            Direction::LEFT,
            Direction::RIGHT,
        ]
    }

    pub fn from<'a, T: Index<usize>>(&self, cell: &GridCell<'a, T>) -> Option<GridCell<'a, T>> {
        cell.go(self)
    }
}

pub type Step = [Direction];

impl<'a, T: Index<usize>> GridCell<'a, T> {
    pub fn value(&self) -> &T::Output {
        self.grid.value_at_unchecked(self.x, self.y)
    }
    pub fn up(&self) -> Option<Self> {
        if self.y == 0 {
            None
        } else {
            self.grid.cell_at(self.x, self.y - 1)
        }
    }

    pub fn down(&self) -> Option<Self> {
        self.grid.cell_at(self.x, self.y + 1)
    }

    pub fn left(&self) -> Option<Self> {
        if self.x == 0 {
            None
        } else {
            self.grid.cell_at(self.x - 1, self.y)
        }
    }

    pub fn right(&self) -> Option<Self> {
        self.grid.cell_at(self.x + 1, self.y)
    }

    pub fn go(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::UP => self.up(),
            Direction::DOWN => self.down(),
            Direction::LEFT => self.left(),
            Direction::RIGHT => self.right(),
        }
    }

    pub fn step(&self, step: &Step) -> Option<Self> {
        // let mut cell = *self;
        // for direction in step {
        //     cell = cell.go(direction)?;
        // }
        // Some(cell)

        step.iter()
            .try_fold(*self, |cell, direction| cell.go(direction))
    }
}

impl<'a, T: Index<usize>> Clone for GridCell<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'a, T: Index<usize>> Copy for GridCell<'a, T> {}

impl<'a, T: Index<usize>> Deref for GridCell<'a, T> {
    type Target = T::Output;

    fn deref(&self) -> &Self::Target {
        self.value()
    }
}
