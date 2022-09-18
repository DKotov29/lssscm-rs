use crate::NumberError::{Parse, UnImplemented};
use itertools::{EitherOrBoth, Itertools};
use std::fmt::{Display, Formatter, LowerHex};
use std::num::ParseIntError;
use std::ops::Add;

pub struct Number {
    v: Vec<u64>,
}

#[derive(Debug)]
pub enum NumberError {
    UnImplemented,
    Parse,
}

impl LowerHex for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in self.v.iter() {
            write!(f, "{:016x} ", i)?;
        }
        Ok(())
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.v)
    }
}

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        let mut v: Vec<u64> = Vec::new();
        let mut carry: bool = false;
        for i in self.v.iter().zip_longest(rhs.v.iter()) {
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
            println!("{}, {}", left, right);
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

impl From<ParseIntError> for NumberError {
    fn from(_item: ParseIntError) -> NumberError {
        Parse
    }
}

impl Number {
    pub fn get_from_str(s: &str) -> Result<Number, NumberError> {
        let (prefix, num) = s.trim().split_at(2);
        let mut vec = Vec::new();
        match prefix {
            "0b" => Err(UnImplemented),
            "0x" => {
                let chunks = num.chars().chunks(16);
                for i in &chunks {
                    let m = i.collect::<String>();
                    //println!("2 \"{}\" \"{}\"", m, u64::from_str_radix(m.as_str(), 16).unwrap());
                    vec.push(u64::from_str_radix(m.as_str(), 16)?);
                }
                Ok(Number { v: vec })
            }
            "0o" => Err(UnImplemented),
            _ => Err(UnImplemented),
        }
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
