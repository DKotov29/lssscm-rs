use std::mem;
use crate::{from_arr_to_string, from_hex_string_to_arr};
//
// const P: usize = 233;
//
// pub fn name() {
//     let mut array: [bool; 233+1] = [false; 234];
//     array[P - 233] = true;
//     array[P - 9] = true;
//     array[P - 4] = true;
//     array[P - 1] = true;
//     array[P - 0] = true;
// }


pub fn add_right_zero(a: &str, i: usize) -> String {
    let mut a = String::from(a);
    (0..i).for_each(|_| a.push('0'));
    a
}

pub fn mul(a: &str, b: &str) -> String {
    let mut res = String::new();
    (0..234).for_each(|_| res.push('0'));
    for i in 0..233 {
        if b.chars().nth(233 - i - 1).unwrap_or('0') == '0' {
            continue;
        }
        res = add(add_right_zero(a, i).as_str(),res.as_str())
    }
    modd(res.as_str())
}

pub fn modd(a: &str) -> String {
    if a.len() == 233 { return a.to_string(); }
    let mut modd= String::from("1");
    modd = add_right_zero(modd.as_str(), 233);
    let mut res = a.to_string();
    while res.len() > 233 {
        while res.chars().nth(0) == Some('0') {
            res = String::from(&res[1..]);
            if res.len() == 233 {
                return res;
            }
        }
        let reserve = modd.clone();
        add_right_zero(reserve.as_str(), res.len() - modd.len());
        res = add(res.as_str(), reserve.as_str());
    }
    res
}
// pub fn mul(a: &str, b: &str) -> String { todo
//     if !a.chars().into_iter().all(|char| char == '1' || char == '0') {
//         panic!("in mul func got not binary number");
//     }
//     let mut a = from_hex_string_to_arr(a);
//     a.reverse();
//     let mut result = Vec::new();
//     let mut carry = 0i8;
//     for i in 0..a.len() {
//         let temp = (*a.get(i).unwrap()) as usize * b as usize + carry as usize;
//         result.push((temp % 2) as i8);
//         carry = (temp / 2) as i8;
//     }
//     if carry != 0 {
//         result.push(carry);
//     }
//     result.reverse();
//     from_arr_to_string(result)
// }

pub fn add(a: &str, b: &str) -> String {
    let mut a = a;
    let mut b = b;
    let mut res = String::new();
    if a.len() < b.len() {
        mem::swap(&mut a, &mut b);
    }
    for i in 0..b.len() {
        let rrres = char::from_u32(a.chars().nth(i).unwrap_or('0').to_digit(2).unwrap_or(0)
            ^ b.chars().nth(i).unwrap_or('0').to_digit(2).unwrap_or(0) + 48).unwrap_or('0');
        res.push(rrres);
    }
    res.push_str(&a[b.len()..]);
    res
}