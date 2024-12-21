use std::fmt::Display;

use crate::quant::{NumConsts, RemEuclid};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2D<N> {
    pub x: N,
    pub y: N,
}

impl<N> Display for Point2D<N>
where
    N: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<N> Point2D<N> {
    pub fn new(x: N, y: N) -> Self {
        Point2D { x, y }
    }

    pub fn rem_euclid(&self, other: Self) -> Self
    where
        N: RemEuclid + Copy,
    {
        Self::new(self.x.rem_euclid(other.x), self.y.rem_euclid(other.y))
    }
}

impl<N, T> Point2D<N>
where
    N: std::ops::Add<Output = T>,
    N: Copy,
{
    pub fn sum(&self) -> T {
        self.x + self.y
    }
}

impl<N> From<(N, N)> for Point2D<N> {
    fn from((x, y): (N, N)) -> Self {
        Point2D { x, y }
    }
}

impl<N> Default for Point2D<N>
where
    N: Default,
{
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl<N> Point2D<N>
where
    N: NumConsts,
{
    pub const fn zero() -> Self {
        Point2D {
            x: N::ZERO,
            y: N::ZERO,
        }
    }

    pub const fn one() -> Self {
        Point2D {
            x: N::ONE,
            y: N::ZERO,
        }
    }
}

impl<N, T> std::ops::Add for Point2D<N>
where
    N: std::ops::Add<Output = T>,
{
    type Output = Point2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<N, T> std::ops::Add<(N, N)> for Point2D<N>
where
    N: std::ops::Add<Output = T>,
{
    type Output = Point2D<T>;

    fn add(self, rhs: (N, N)) -> Self::Output {
        self + Point2D::from(rhs)
    }
}

impl<N, T> std::ops::Sub for Point2D<N>
where
    N: std::ops::Sub<Output = T>,
{
    type Output = Point2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<N, T> std::ops::Sub<(N, N)> for Point2D<N>
where
    N: std::ops::Sub<Output = T>,
{
    type Output = Point2D<T>;

    fn sub(self, rhs: (N, N)) -> Self::Output {
        self - Point2D::from(rhs)
    }
}

impl<N, U, T> std::ops::Mul<U> for Point2D<N>
where
    N: std::ops::Mul<U, Output = T>,
    U: Copy,
{
    type Output = Point2D<T>;

    fn mul(self, rhs: U) -> Self::Output {
        Point2D::new(self.x * rhs, self.y * rhs)
    }
}

impl<N, U, T> std::ops::Div<U> for Point2D<N>
where
    N: std::ops::Div<U, Output = T>,
    U: Copy,
{
    type Output = Point2D<T>;

    fn div(self, rhs: U) -> Self::Output {
        Point2D::new(self.x / rhs, self.y / rhs)
    }
}

impl<N, T> std::ops::Rem for Point2D<N>
where
    N: std::ops::Rem<Output = T>,
{
    type Output = Point2D<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Point2D::new(self.x % rhs.x, self.y % rhs.y)
    }
}

pub type Point2Disize = Point2D<isize>;
pub type Point2Dusize = Point2D<isize>;
