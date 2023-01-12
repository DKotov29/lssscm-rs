extern crate core;

use crate::NumberError::{Parse, UnImplemented};
use itertools::{EitherOrBoth, Itertools};
use std::cmp::max;
use std::fmt::{Binary, Debug, Display, format, Formatter, LowerHex};
use std::num::ParseIntError;
use std::ops::{Add, Mul, Shl, Shr, Sub};

#[derive(Debug)]
pub struct Number {
    v: Vec<u64>,
}

#[derive(Debug)]
pub enum NumberError {
    UnImplemented,
    Parse,
}

#[derive(Debug)]
pub struct OverflowSub;

impl LowerHex for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in self.v.iter().rev() {
            s.push_str(format!("{:0>16x}", i).as_str());
        }
        write!(f, "{}", s.trim_start_matches("0"))?;

        Ok(())
    }
}

// impl Binary for Number {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         for i in self.v.iter() {
//             write!(f, "{:064b}", i.reverse_bits())?;
//         }
//         Ok(())
//     }
// }

impl Add<&Number> for &Number {
    type Output = Number;

    fn add(self, rhs: &Number) -> Self::Output {
        let mut v: Vec<u64> = Vec::with_capacity(max(self.v.len(), rhs.v.len()));
        let mut carry: bool = false;
        for i in self.v.iter().zip_longest(rhs.v.iter()) {
            let mut left: u64 = 0;
            let mut right: u64 = 0;
            match i {
                EitherOrBoth::Both(x, y) => {
                    left = (*x);
                    right = (*y);
                }
                EitherOrBoth::Left(x) | EitherOrBoth::Right(x) => {
                    left = (*x);
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
            v.push(1u64);
        }
        Number { v }
    }
}

impl Sub<&Number> for &Number {
    type Output = Result<Number, OverflowSub>;

    fn sub(self, rhs: &Number) -> Self::Output {
        let mut v: Vec<u64> = Vec::with_capacity(max(self.v.len(), rhs.v.len()));
        let mut carry: bool = false;
        for i in self.v.iter().zip_longest(rhs.v.iter()) {
            let mut left: u64 = 0;
            let mut right: u64 = 0;
            match i {
                EitherOrBoth::Both(x, y) => {
                    left = *x;
                    right = *y;
                }
                EitherOrBoth::Left(x) => left = *x,
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
        if carry {
            return Err(OverflowSub);
        }
        Ok(Number { v })
    }
}

impl From<ParseIntError> for NumberError {
    fn from(_item: ParseIntError) -> NumberError {
        Parse
    }
}

impl PartialEq<Self> for Number {
    fn eq(&self, other: &Self) -> bool {
        let mut res = true;
        for x in self.v.iter().zip_longest(other.v.iter()) {
            match x {
                EitherOrBoth::Both(a, b) => {
                    if a != b { res = false; break; }
                }
                EitherOrBoth::Left(&a) | EitherOrBoth::Right(&a) =>
                    {if a == 0u64 { res = false; break; }}
            }
        }
        res
    }
}

impl Mul for &Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Number {
        let se = self.clone();
        let s2 = format!("{:x}", rhs);
        let mut res = Number::zero();
        for (i, ii) in from_hex_to_bin(s2.as_str()).chars().rev().enumerate() {
            if ii == '1' {
                let nowww = se.clone() << i as u32;
                res = &res + &nowww;
            }
        }
        res
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        Number{ v: self.v.clone() }
    }
}

impl Shl<u32> for Number {
    type Output = Number;

    fn shl(self, rhs: u32) -> Self::Output {
        if rhs == 0 { return  self; }
        let shl_max_63 = |num: Number, rhs :u32| {
            let mut vec = Vec::with_capacity(num.v.len());
            let right_shift = 64 - rhs;
            let mut other: u64 = 0;
            for i in num.v {
                vec.push((i.wrapping_shl(rhs)) | other);
                other = i.wrapping_shr( right_shift);
            }
            vec.push(other);
            Number { v: vec }
        };
        if rhs > 63 {
            let mut save = self;
            for _ in 0..rhs / 63 {
                save = shl_max_63(save, 63u32);
            }
            save = shl_max_63(save, rhs % 63);
            save
        } else {
            shl_max_63(self, rhs)
        }
    }
}

impl Shr<u32> for Number {
    type Output = Number;

    fn shr(self, rhs: u32) -> Self::Output {
        if rhs == 0 { return  self; }
        let mut f = format!("{:#x}", self);
        f = from_hex_to_bin(f.trim_start_matches("0"));
        f = f.chars().take(f.len()-(rhs as usize)).collect::<String>();
        let mut b = String::from("0b");
        b.push_str(f.as_str());
        Number::get_from_str(b.as_str()).unwrap()
    }
}

pub fn from_hex_to_bin(s: &str) -> String {
    let mut sss = String::new();
    for c in s.to_uppercase().chars() {
        let ddd = match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        };
        sss.push_str(ddd);
    }
    sss
}

impl Number {
    pub fn get_from_str(s: &str) -> Result<Number, NumberError> {
        let (prefix, num) = s.trim().split_at(2);
        let mut vec;
        if num.is_empty() {
            return Ok(Number::zero());
        }
        match prefix {
            "0b" => {
                vec = Vec::new();
                let chunks = num.chars().rev().chunks(64);
                for i in &chunks {
                    let m = i.collect::<String>().chars().rev().collect::<String>();
                    vec.push(u64::from_str_radix(m.as_str(), 2)?);
                }
                Ok(Number { v: vec })
            },
            "0x" => {
                vec = Vec::new();
                let chunks = num.chars().rev().chunks(16);
                for i in &chunks {
                    let m = i.collect::<String>().chars().rev().collect::<String>();
                    vec.push(u64::from_str_radix(m.as_str(), 16)?);
                }
                Ok(Number { v: vec })
            }
            _ => Err(UnImplemented),
        }
    }

    pub fn zero() -> Number {
        Number { v: vec![0u64] }
    }

    pub fn one() -> Number {
        // maybe not
        let mut v = Vec::new();
        v.push(1u64.reverse_bits());
        Number { v }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
