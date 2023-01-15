
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater};
use std::mem;
use crate::{from_arr_to_string, from_hex_string_to_arr};

pub fn add_right_zero(a: &str, i: usize) -> String {
    let mut a = String::from(a);
    (0..i).for_each(|_| a.push('0'));
    a
}

fn compare(a: &str, b: &str) -> Ordering {
    for i in 0..a.len().max(b.len()) {
        let aa = a.chars().nth(i);
        let bb = b.chars().nth(i);
        if aa.is_some() && bb.is_none() {
            return Greater;
        }
        if bb.is_some() && aa.is_none() {
            return Ordering::Less;
        }
        if aa.is_some() && bb.is_some() && aa.unwrap() == '1' && bb.unwrap() == '0' {
            return Ordering::Greater;
        }
        if aa.is_some() && bb.is_some() && bb.unwrap() == '1' && aa.unwrap() == '0' {
            return Ordering::Less;
        }
    }
    Equal
}

pub fn mul(mut a: &str, mut b: &str, c: &str) -> String {
    let mut a = a.trim_start_matches("0").to_string();
    let mut b = b.trim_start_matches("0").to_string();
    let mut res = String::from("0");
    for i in 0..b.len() - 1 {
        match char_to_i(b.chars().nth(i).unwrap_or('0')) {
            0 => {
                if is_zero(b.as_str()) {
                    let mut s = String::new();
                    (0..b.len() + i).for_each(|_| s.push('0'));
                    b = s
                }
            }
            1 => {
                res = add(res.as_str(),
                          add_right_zero(a.as_str(), b.len() - i - 1).as_str())
            }
            _ => {}
        }
    }
    println!("{res}");
    modd(res.as_str(), c)
// res
}

pub fn modd(a: &str, c: &str) -> String {
    let mut res = a.to_string();
    if compare(a, c) == Ordering::Less {
        return a.to_string();
    }
    while compare(res.as_str(), c) == Greater || compare(res.as_str(), c) == Equal {
        let d = add_right_zero(c, res.len() - c.len());
        res = sub(res.as_str(), d.as_str());
        res = res.trim_start_matches("0").to_string();
    }
    res
}

// pub fn modd(a: &str) -> String {
//     if a.len() == 233 { return a.to_string(); }
//     // let mut modd = String::from("100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000100011");
//     let mut modd = String::from("1");
//     modd = add_right_zero(modd.as_str(), 233);
//     let mut res = a.to_string();
//     while res.len() > 233 {
//         while let Some(stripped) = res.strip_prefix("0") {
//             res = String::from(stripped);
//             if res.len() == 233 {
//                 return res;
//             }
//         }
//         let mut reserve = add_right_zero(modd.as_str(), res.len() - modd.len() - 1);
//         res = add(res.chars().as_str(), reserve.as_str());
//
//         println!("reverve: {reserve} \nres: {res} \n");
//     }
//     res
// }

pub fn sub(a: &str, b: &str) -> String {
    let a = a.chars().rev().collect::<String>();
    let b = b.chars().rev().collect::<String>();
    let mut res = String::new();
    for i in 0..b.len() {
        if a.chars().nth(i).unwrap_or('0') == '1' &&
            b.chars().nth(i).unwrap_or('0') == '0' ||
            (a.chars().nth(i).unwrap_or('0') == '0' &&
                b.chars().nth(i).unwrap_or('0') == '1' && i != b.len() - 1) {
            res.push('1');
        } else {
            res.push('0');
        }
    }
    res.push_str(&a.as_str()[b.len()..]);
    res.chars().rev().collect::<String>()
}

pub fn add(a: &str, b: &str) -> String {
    let mut a = a.chars().rev().collect::<String>();
    let mut b = b.chars().rev().collect::<String>();
    let mut res = String::new();
    if a.len() < b.len() {
        mem::swap(&mut a, &mut b);
    }
    for i in 0..b.len() {
        let rrres = char_to_i(a.chars().nth(i).unwrap_or('0'))
            ^ char_to_i(b.chars().nth(i).unwrap_or('0'));
        res.push_str(rrres.to_string().chars().as_str());
    }
    res.push_str(&a[b.len()..]);

    res.chars().rev().collect::<String>()
}

pub fn is_zero(a: &str) -> bool {
    a.chars().all(|char| char == '0')
}

pub fn char_to_i(i: char) -> u8 {
    match i {
        '0' => 0,
        '1' => 1,
        _ => panic!("in char to int function there are problem")
    }
}