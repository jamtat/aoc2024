use std::ops::{Div, Mul, Rem};

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Mul<Output = T> + Div<Output = T> + Eq + NumConsts + Rem<Output = T> + Copy,
{
    a * b / gcd(a, b)
}

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Rem<Output = T> + Eq + NumConsts + Copy,
{
    if b == T::ZERO {
        a
    } else {
        gcd(b, a % b)
    }
}

pub trait QuantIter: Iterator {
    fn lcm(mut self) -> Option<Self::Item>
    where
        Self::Item: Mul<Output = Self::Item>
            + Div<Output = Self::Item>
            + Eq
            + NumConsts
            + Rem<Output = Self::Item>
            + Copy,
        Self: Sized,
    {
        self.next().map(|first| self.fold(first, lcm))
    }

    fn gcd(mut self) -> Option<Self::Item>
    where
        Self::Item: Rem<Output = Self::Item> + Eq + NumConsts + Copy,
        Self: Sized,
    {
        self.next().map(|first| self.fold(first, gcd))
    }
}

impl<T: ?Sized> QuantIter for T where T: Iterator {}

pub trait NumConsts {
    const ZERO: Self;
    const ONE: Self;
}
macro_rules! impl_consts {
    ($typ:ty) => {
        impl NumConsts for $typ {
            const ZERO: $typ = 0;
            const ONE: $typ = 1;
        }
    };
    ($typ:ty, f) => {
        impl NumConsts for $typ {
            const ZERO: $typ = 0.0;
            const ONE: $typ = 1.0;
        }
    };
}
impl_consts!(u8);
impl_consts!(u16);
impl_consts!(u32);
impl_consts!(u64);
impl_consts!(u128);
impl_consts!(usize);
impl_consts!(i8);
impl_consts!(i16);
impl_consts!(i32);
impl_consts!(i64);
impl_consts!(i128);
impl_consts!(isize);
impl_consts!(f32, f);
impl_consts!(f64, f);
