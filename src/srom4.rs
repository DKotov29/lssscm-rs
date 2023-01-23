use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater};
use std::collections::HashSet;
use Ordering::Less;

const GEN: [usize; 5] = [233, 9, 4, 1, 0];
static mut Matrix: Vec<Vec<i16>> = Vec::new();

pub fn add_right_zero(a: &str, i: usize) -> String {
    let mut a = String::from(a);
    (0..i).for_each(|_| a.push('0'));
    a
}

pub unsafe fn mul(a: &str, b: &str) -> String {
    let mut a = a.to_string();
    let mut b = b.to_string();
    let mut result = Vec::with_capacity(233);
    for i in 0..233 {
        let oh = mul_matrix(
            &mul_matrix(&bin_str_to_vec(a.as_str()), &Matrix),
            &transpose(&bin_str_to_vec(b.as_str())),
        );
        result.push(oh[0][0]);
        a = cshl(a.as_str());
        b = cshl(b.as_str());
    }
    result.iter().map(|i| i.to_string()).collect::<String>()
}

pub fn bin_str_to_vec(a: &str) -> Vec<Vec<i16>> {
    let mut result = Vec::new();
    (0..233).for_each(|_| {
        let mut vec = Vec::new();
        (0..233).for_each(|_| vec.push(0i16));
        result.push(vec)
    });
    for (i, u) in a.chars().map(|char| char_to_i(char)).enumerate() {
        match u {
            1 => {
                result[0][i] = 1;
            }
            _ => {}
        }
    }
    result
}

pub fn transpose(a: &Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    let mut result = Vec::with_capacity(a.first().unwrap().len());
    (0..a.first().unwrap().len()).for_each(|_| {
        let mut vec = Vec::new();
        (0..a.len()).for_each(|_| vec.push(0i16));
        result.push(vec)
    });
    for i in 0..a.len() {
        for j in 0..a.first().unwrap().len() {
            result[j][i] = a[i][j];
        }
    }
    result
}

pub fn mul_matrix(a: &Vec<Vec<i16>>, b: &Vec<Vec<i16>>) -> Vec<Vec<i16>> {
    let a_col_len = a.first().unwrap().len();
    let b_row_len = b.len();
    if a_col_len != b_row_len {
        panic!("we can't multiply matrix with this lens")
    }
    let a_row_len = a.len();
    let b_col_len = b.first().unwrap().len();
    let mut res = Vec::new();
    (0..a_row_len).for_each(|_| {
        let mut vec = Vec::with_capacity(b_col_len);
        (0..b_col_len).for_each(|_| vec.push(0i16));
        res.push(vec)
    });
    for i in 0..a_row_len {
        for j in 0..b_col_len {
            for k in 0..a_col_len {
                res[i][j] += a[i][k] * b[k][j];
                res[i][j] %= 2;
            }
        }
    }
    res
}

pub fn cshl(a: &str) -> String {
    let mut res = String::from(&a[..a.len()-1]);
    res.push_str(&a[a.len()-1..]);
    res
}

pub unsafe fn matrix_find() {
    let p: i128 = 2 * 233 + 1;
    // let exist_onb = {
    //     let is_prime = {
    //         let mut pr = true;
    //         for i in 2..p {
    //             if (p % i) == 0 {
    //                 pr = false;
    //                 break;
    //             }
    //         }
    //         pr
    //     };
    //     if is_prime {
    //         if 2i128.pow(2 * 233) % p == 1 {
    //             true
    //         } else {
    //             p % 4 == 3 && 2i128.pow(233) % p == 1
    //         }
    //     } else {
    //         false
    //     }
    // };
    // if !exist_onb {
    //     panic!("no...");
    // }
    Matrix = Vec::new();
    (0..233).for_each(|_| {
        let mut vec = Vec::with_capacity(233);
        (0..233).for_each(|_| vec.push(0i16));
        Matrix.push(vec)
    });

    for i in 0..233 {
        for j in 0..233 {
            let c1 = 2f64.powf(i as f64) + 2f64.powf(j as f64);
            let mut c2 = 2f64.powf(i as f64) - 2f64.powf(j as f64);
            let mut c3 = -2f64.powf(i as f64) + 2f64.powf(j as f64);
            let c4 = (-22f64.powf(i as f64) - 2f64.powf(j as f64)) % p as f64 + p as f64;
            if c2 < 0f64 {
                c2 = c2 % p as f64 + p as f64;
            }
            if c3 < 0f64 {
                c3 = c3 % p as f64 + p as f64;
            }

            if c1 % p as f64 == 1 as f64
                || c2 % p as f64 == 1 as f64
                || c3 % p as f64== 1 as f64
                || c4 % p as f64 == 1 as f64
            {
                Matrix[i as usize][j as usize] = 1;
            }
            // else {
            //     Matrix[i as usize][j as usize] = 0;
            // }
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
    for i in (0..=first).rev()
    {
        if a.contains(&i) {
            ss.push('1');
        } else {
            ss.push('0');
        }
    }
    ss.chars().collect::<String>()
}

pub fn add(a1: &str, b1: &str) -> String {
    let a = convert_to_vec(a1);
    let b = convert_to_vec(b1);
    let mut res = HashSet::new();
    a.iter().chain(b.iter()).for_each(|&each| {
        if !res.remove(&each) {
            res.insert(each);
        }
    });
    let mut res = res.into_iter().collect::<Vec<usize>>();
    res.sort();
    res.reverse();
    convert_to_string(res)
}

pub fn char_to_i(i: char) -> u8 {
    match i {
        '0' => 0,
        '1' => 1,
        _ => panic!("in char to int function there are problem"),
    }
}
