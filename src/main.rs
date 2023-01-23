extern crate core;

use std::cmp::Ordering;

mod srom2;

pub use srom2::*;

mod srom3;

pub use srom3::*;

mod srom4;
mod test;

pub use srom4::*;

fn main() {
    unsafe {
        matrix_find();
        assert_eq!(srom4::add("101101", "111010"), "10111"); // result is 010111 but func trim 0
        return;
    }
}

fn add(first: &str, second: &str) -> String {
    if first.eq("0") {
        return String::from(second);
    }
    if second.eq("0") {
        return String::from(first);
    }
    let a = from_hex_string_to_arr(first);
    let b = from_hex_string_to_arr(second);
    let max = a.len().max(b.len());
    let mut result = Vec::new();
    let mut carry = 0i8;
    for i in 1..=max {
        let a1 = if a.len() >= i {
            a.get(a.len() - i).unwrap_or(&0).clone()
        } else {
            0
        };
        let b1 = if b.len() >= i {
            b.get(b.len() - i).unwrap_or(&0).clone()
        } else {
            0
        };
        let temp = a1 + b1 + carry;
        result.push(temp & 0b1111);
        carry = temp >> 4;
    }
    if carry > 0 {
        result.push(carry);
    }
    result.reverse();
    from_arr_to_string(result)
}

fn sub(a: &str, b: &str) -> String {
    match compare(a, b) {
        Ordering::Less => {
            panic!("second number greater than first")
        }
        Ordering::Equal => return "0".to_string(),
        Ordering::Greater => {}
    }
    let a = from_hex_string_to_arr(a);

    let b = from_hex_string_to_arr(b);
    let max = a.len().max(b.len());
    let mut result = Vec::new();
    let mut borrow = 0i8;
    for i in 1..=max {
        let a1 = if a.len() >= i {
            a.get(a.len() - i).unwrap_or(&0).clone()
        } else {
            0
        };
        let b1 = if b.len() >= i {
            b.get(b.len() - i).unwrap_or(&0).clone()
        } else {
            0
        };
        let temp = (a1 as i32) - (b1 as i32) - (borrow as i32);
        if temp >= 0 {
            borrow = 0;
            result.push(temp);
        } else {
            result.push(temp & 15);
            borrow = 1;
        }
    }
    result.reverse();
    from_arr_to_string(
        result
            .iter()
            .map(|&a| a as u8)
            .map(|a| a as i8)
            .collect::<Vec<i8>>(),
    )
}

fn mul_digit(a: &str, b: i8) -> String {
    let mut a = from_hex_string_to_arr(a);
    a.reverse();
    let mut result = Vec::new();
    let mut carry = 0i8;
    for i in 0..a.len() {
        let temp = (*a.get(i).unwrap()) as usize * b as usize + carry as usize;
        result.push((temp % 16) as i8);
        carry = (temp / 16) as i8;
    }
    if carry != 0 {
        result.push(carry);
    }
    result.reverse();
    from_arr_to_string(result)
}

fn mul(a: &str, b: &str) -> String {
    let mut result = "0".to_string();
    let mut bb = from_hex_string_to_arr(b);
    bb.reverse();
    let mut temp;
    for (ii, &i) in bb.iter().enumerate() {
        temp = mul_digit(a, i);
        temp = add_right_zero(temp.as_str(), ii);
        result = add(result.as_str(), temp.as_str());
    }
    result
}

fn add_right_zero(a: &str, i: usize) -> String {
    let mut a = String::from(a);
    (0..i).for_each(|_| a.push('0'));
    a
}

fn compare(a: &str, b: &str) -> Ordering {
    // return Ordering::Greater;
    let a = from_hex_string_to_arr(a);
    let b = from_hex_string_to_arr(b);

    for i in 0..a.len().max(b.len()) {
        let aa = a.get(i);
        let bb = b.get(i);
        if aa.is_some() && bb.is_none() {
            return Ordering::Greater;
        }
        if bb.is_some() && aa.is_none() {
            return Ordering::Less;
        }
        if aa.is_some() && bb.is_some() && aa.unwrap() > bb.unwrap() {
            return Ordering::Greater;
        }
        if aa.is_some() && bb.is_some() && bb.unwrap() > aa.unwrap() {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn remove_last_digits(s: &str, num: usize) -> String {
    String::from(&s[..s.len() - num])
}

fn from_hex_string_to_arr(string: &str) -> Vec<i8> {
    if string.eq("0") {
        return vec![0];
    }
    string
        .trim_start_matches('0')
        .chars()
        .into_iter()
        .map(|char| match char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' | 'a' => 10,
            'B' | 'b' => 11,
            'C' | 'c' => 12,
            'D' | 'd' => 13,
            'E' | 'e' => 14,
            'F' | 'f' => 15,
            a => panic!("{a} error in from_string_to_arr"),
        })
        .collect::<Vec<i8>>()
}

fn from_arr_to_string(vec: Vec<i8>) -> String {
    vec.iter()
        .map(|&u| match u {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            a => panic!("{a} error in from_arr_to_string"),
        })
        .collect::<String>()
        .trim_start_matches("0")
        .to_string()
}

pub fn pow(a: &str, i: u8) -> String {
    let mut res = a.to_string();
    if i == 0 { return "1".to_string(); }
    if i == 1 { return a.to_string(); }

    (0..i - 1).for_each(|_| { res = mul(res.as_str(), a) });
    res
}
