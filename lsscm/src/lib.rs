extern crate core;

use std::cmp::max;
use crate::NumberError::{Parse, UnImplemented};
use itertools::{EitherOrBoth, Itertools};
use std::fmt::{Binary, Debug, Display, Formatter, LowerHex};
use std::num::ParseIntError;
use std::ops::{Add, Shl, Shr, Sub};

pub struct Number {
    v: Vec<u64>,
}

#[derive(Debug)]
pub enum NumberError {
    UnImplemented,
    Parse,
}

pub struct OverflowSub;

impl LowerHex for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in self.v.iter().rev() {
            write!(f, "{:016x}", i)?;
        }
        Ok(())
    }
}

impl Binary for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in self.v.iter().rev() {
            write!(f, "{:064b}", i)?;
        }
        Ok(())
    }
}

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        let mut v: Vec<u64> = Vec::with_capacity(max(self.v.len(), rhs.v.len()));
        let mut carry: bool = false;
        for i in self.v.iter().rev().zip_longest(rhs.v.iter().rev()) {
            let mut left: u64 = 0;
            let mut right: u64 = 0;
            match i {
                EitherOrBoth::Both(x, y) => {
                    left = *x;
                    right = *y;
                }
                EitherOrBoth::Left(x) | EitherOrBoth::Right(x) => {
                    left = *x;
                }
            }
            if carry {
                let (a, b) = left.overflowing_add(1);
                if b {
                    v.push(right);
                    carry = true;
                } else {
                    let (res, car) = a.overflowing_add(right);
                    v.push(res);
                    carry = car;
                }
            } else {
                let (res, car) = left.overflowing_add(right);
                v.push(res);
                carry = car;
            }
        }
        if carry {
            v.push(1);
        }
        Number { v }
    }
}

//todo
impl Sub<&Number> for &Number {
    type Output = Result<Number, OverflowSub>;

    fn sub(self, rhs: &Number) -> Self::Output {
        let mut v: Vec<u64> = Vec::with_capacity(max(self.v.len(), rhs.v.len()));
        let mut carry: bool = false;
        for i in self.v.iter().rev().zip_longest(rhs.v.iter()) {
            let mut left: u64 = 0;
            let mut right: u64 = 0;
            match i {
                EitherOrBoth::Both(x, y) => {
                    left = *x;
                    right = *y;
                }
                EitherOrBoth::Left(x) => {
                    left = *x
                }
                EitherOrBoth::Right(x) => {
                    right = *x;
                }
            }
            if carry {
                let (a, b) = left.overflowing_sub(1);
                if b {
                    v.push(!right);
                    carry = true;
                } else {
                    let (res, car) = a.overflowing_sub(right);
                    v.push(res);
                    carry = car;
                }
            } else {
                let (res, car) = left.overflowing_sub(right);
                v.push(res);
                carry = car;
            }
        }
        // if carry { return Err(OverflowSub) }
        Ok(Number { v })
    }
}

impl From<ParseIntError> for NumberError {
    fn from(_item: ParseIntError) -> NumberError {
        Parse
    }
}

// todo not work correct for > 64
impl Shl<usize> for Number {
    type Output = Number;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut vec = Vec::with_capacity(self.v.len());
        let right_shift = 64 - rhs;
        let mut other: u64 = 0;
        for i in self.v {
            vec.push((i << rhs) | other);
            other = i >> right_shift;
        }
        vec.push(other);
        Number { v: vec }
    }
}

// todo not work correct for > 64
impl Shr<usize> for Number {
    type Output = Number;

    fn shr(self, rhs: usize) -> Self::Output {
        // if rhs > 64 {
        //
        // }
        let left_shift = 64 - rhs;
        let mut other: u64 = 0;
        Number {
            v: self.v.iter().rev().map(|f| {
                let m = (f >> rhs) | other;
                other = f << left_shift;
                m
            }).rev().collect::<Vec<u64>>()
        }
    }
}

impl Number {
    pub fn get_from_str(s: &str) -> Result<Number, NumberError> {
        let (prefix, num) = s.trim().split_at(2);
        let mut vec = Vec::new();
        match prefix {
            "0b" => {
                if num.is_empty() { return Ok(Number::zero()); }
                let chunks = num.chars().chunks(64);
                for i in &chunks {
                    let m = i.collect::<String>();
                    vec.push(u64::from_str_radix(m.as_str(), 2)?);
                }
                Ok(Number { v: vec })
            }
            "0x" => {
                if num.is_empty() { return Ok(Number::zero()); }
                let chunks = num.chars().chunks(16);
                for i in &chunks {
                    let m = i.collect::<String>();
                    println!("{m}" );
                    vec.push(u64::from_str_radix(m.as_str(), 16)?);
                }
                Ok(Number { v: vec })
            }
            _ => Err(UnImplemented),
        }
    }

    pub fn zero() -> Number {
        let mut v = Vec::new();
        v.push(0u64);
        Number { v }
    }

    pub fn one() -> Number {
        let mut v = Vec::new();
        v.push(1u64);
        Number { v }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
