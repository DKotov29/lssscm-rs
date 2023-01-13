extern crate core;

use itertools::{EitherOrBoth, Itertools, ZipLongest};
use std::cmp::Ordering;
use std::slice::Iter;

mod test;
mod srom3;

fn main() {
    println!("{:?}", from_hex_string_to_arr("60"));
    println!("{}", sub("120", "60"));

    // println!("{}", sub("D4D2110984907B5625309D956521BAB4157B8B1ECE04043249A3D379AC112E5B9AF44E721E148D88A942744CF56A06B92D28A0DB950FE4CED2B41A0BD38BCE7D0BE1055CF5DE38F2A588C2C9A79A75011058C320A7B661C6CE1C36C7D870758307E5D2CF07D9B6E8D529779B6B2910DD17B6766A7EFEE215A98CAC300F2827DB",
    //                    "3A7EF2554E8940FA9B93B2A5E822CC7BB262F4A14159E4318CAE3ABF5AEB1022EC6D01DEFAB48B528868679D649B445A753684C13F6C3ADBAB059D635A2882090FC166EA9F0AAACD16A062149E4A0952F7FAAB14A0E9D3CB0BE9200DBD3B0342496421826919148E617AF1DB66978B1FCD28F8408506B79979CCBCC7F7E5FDE7"));
}
//  - *
// - для №2 редукція Баррета (+ працюючий мінімум з попередньої лаби, звісно)
fn add(num_1: &str, num_2: &str) -> String {
    if num_1.eq("0") {
        return String::from(num_2);
    }
    if num_2.eq("0") {
        return String::from(num_1);
    }
    let a = from_hex_string_to_arr(num_1);
    let b = from_hex_string_to_arr(num_2);
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
    // todo
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
        let temp = ((a1 as i8) - (b1 as i8) - (borrow as i8));
        println!("{temp}");
        if temp >= 0 {
            borrow = 0;
            println!("push: {temp}");
            result.push(temp);
        } else {
            if max == 1 || (borrow == 1 && max == 1) {
                println!("push: {}", temp.abs());
                result.push(temp.abs());
            } else {
                println!("push: {}", temp + 2);
                result.push(2 + temp.abs());
            }

            borrow = 1;
        }
    }
    result.reverse();
    from_arr_to_string(result.iter().map(|&a| a as i8).collect::<Vec<i8>>())
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
    let mut  bb = from_hex_string_to_arr(b);
    bb.reverse();
    let mut temp;
    for (ii, &i) in bb.iter().enumerate() {
        temp = mul_digit(a, i);
        temp = add_right_zero(temp.as_str(), ii);
        result = add(result.as_str(), temp.as_str());
    }
    result.to_string()
}

fn add_right_zero(a: &str, i: usize) -> String {
    let mut a = String::from(a);
    (0..i).for_each(|_| a.push('0'));
    a
}

fn barret_reduction() {}

fn compare(a: &str, b: &str) -> Ordering {
    // return Ordering::Greater;
    let a = from_hex_string_to_arr(a);
    let b = from_hex_string_to_arr(b);
    if a.len() > b.len() {
        return Ordering::Greater;
    } else if b.len() > a.len() {
        return Ordering::Less;
    } else {
        for i in 0..a.len().max(b.len()) {
            let aa = a.get(i);
            let bb = b.get(i);
            if aa.is_some() && bb.is_none() { return Ordering::Greater; }
            if bb.is_some() && aa.is_none() { return Ordering::Less; }
            if aa.is_some() && bb.is_some() && aa.unwrap() > bb.unwrap() { return Ordering::Greater }
            if aa.is_some() && bb.is_some() && bb.unwrap() > aa.unwrap() { return Ordering::Less }
        }
        todo!()
    }
}

fn remove_last_digits(s: &str, num: usize) -> String {
    String::from(&s[..s.len()-num])
}

fn from_hex_string_to_arr(string: &str) -> Vec<i8> {
    if string.eq("0") {
        return vec![0];
    }
    string
        .trim_start_matches("0")
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
            'A' => 10,
            'B' => 11,
            'C' => 12,
            'D' => 13,
            'E' => 14,
            'F' => 15,
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
            _ => panic!("error in from_arr_to_string"),
        })
        .collect::<String>()
        .trim_start_matches("0")
        .to_string()
}
