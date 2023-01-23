use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater};
use std::collections::HashSet;
use Ordering::Less;


const GEN: [usize; 5] = [233, 9, 4, 1, 0];

pub fn add_right_zero(a: &str, i: usize) -> String {
    let mut a = String::from(a);
    (0..i).for_each(|_| a.push('0'));
    a
}

fn compare(a: &str, b: &str) -> Ordering {
    let a = a.trim_start_matches('0');
    let b = b.trim_start_matches('0');
    for i in 0..a.len().max(b.len()) {
        let aa = a.chars().nth(i);
        let bb = b.chars().nth(i);
        if aa.is_some() && bb.is_none() {
            return Greater;
        }
        if bb.is_some() && aa.is_none() {
            return Less;
        }
        if aa.is_some()
            && bb.is_some()
            && aa.unwrap() == '1'
            && bb.unwrap() == '0'
            && a[i..].len() >= b[i..].len()
        {
            return Greater;
        }
        if aa.is_some()
            && bb.is_some()
            && bb.unwrap() == '1'
            && aa.unwrap() == '0'
            && a[i..].len() <= b[i..].len()
        {
            return Less;
        }
    }
    Equal
}

//          000101      1101010
pub fn mul(mut a: &str, mut b: &str) -> String {
    let mut res = HashSet::new();
    let a = convert_to_vec(a);
    let b = convert_to_vec(b);
    for x in a.iter() {
        for y in b.iter() {
            let reees = *x + *y;
            if !res.remove(&reees) {
                res.insert(reees);
            }
        }
    }
    let mut res = res.into_iter().collect::<Vec<usize>>();
    let mut temp_res = convert_to_string(res.clone());
    res.sort();
    res.reverse();
    println!("vector of stepeney: {res:?}");
    temp_res = modd(temp_res.as_str());

    temp_res
}

pub fn modd(a: &str) -> String {
    println!(
        "{:?}\ngot: {} \ngen: {}\n",
        compare(a, convert_to_string(GEN.to_vec()).as_str()),
        a,
        convert_to_string(GEN.to_vec()).as_str()
    );
    match compare(a, convert_to_string(GEN.to_vec()).as_str()) {
        Greater => {
            let mut temp_res = a.to_string();
            let mut conv = convert_to_vec(a);
            conv.sort();
            conv.reverse();
            let (greater, b) = conv.iter().next().unwrap().overflowing_sub(233);
            if b {
                return temp_res;
            }
            let mut temp = Vec::new();
            for i in GEN {
                temp.push(greater + i);
            }
            // println!("temp: {temp:?}");
            temp_res = add(temp_res.as_str(), convert_to_string(temp).as_str());
            // println!("temp_res: {}", temp_res);
            modd(temp_res.as_str())
        }
        Less => {
            return a.to_string();
        }
        Equal => {
            return "0".to_string();
        }
    }
}

pub fn convert_to_vec(a: &str) -> Vec<usize> {
    let mut res = Vec::new();
    let a = a.chars().rev().collect::<String>();
    for i in (0..a.len()).rev() {
        let a: char = a.to_string().chars().nth(i).unwrap();
        match char_to_i(a) {
            1 => {
                res.push(i as usize);
            }
            _ => {}
        }
    }
    res
}

pub fn convert_to_string(a: Vec<usize>) -> String {
    let mut s = a.clone();
    s.sort();
    let first: usize = *s.iter().rev().next().unwrap();
    let mut ss = String::new();
    for i in (0..=first).rev() {
        if a.contains(&i) {
            ss.push('1');
        } else {
            ss.push('0');
        }
    }
    ss.chars().collect::<String>()
}

// pub fn modd(a: &str) -> {}

pub fn add(a1: &str, b1: &str) -> String {
    // let mut a: String = a.chars().rev().collect::<String>();
    // let mut b: String = b.chars().rev().collect::<String>();
    // let mut res = String::new();
    //
    // for i in 0..(b.len().max(a.len())) {
    //     let rrres = char_to_i(a.chars().nth(i).unwrap_or('0'))
    //         ^ char_to_i(b.chars().nth(i).unwrap_or('0'));
    //     res.push_str(rrres.to_string().chars().as_str());
    // }
    // let res =  res.chars().rev().collect::<String>().trim_start_matches('0').to_string();
    // println!("a:{a} \n+\nb:{b}\nc:{res}");
    // res
    let a = convert_to_vec(a1);
    println!("{a:?}");
    let b = convert_to_vec(b1);
    println!("{b:?}");
    let mut res = HashSet::new();
    a.iter().chain(b.iter()).for_each(|&each| {
        if !res.remove(&each) {
            res.insert(each);
        }
    });
    let mut res = res.into_iter().collect::<Vec<usize>>();
    res.sort();
    res.reverse();
    println!("stepenya: {res:?}");
    let res = convert_to_string(res);
    println!("a:{a1} \n+\nb:{b1}\nc:{res}");
    res
}

pub fn is_zero(a: &str) -> bool {
    a.chars().all(|char| char == '0')
}

pub fn char_to_i(i: char) -> u8 {
    match i {
        '0' => 0,
        '1' => 1,
        _ => panic!("in char to int function there are problem"),
    }
}
