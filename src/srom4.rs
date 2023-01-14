use std::mem;

pub fn add(a: &str, b: &str, n: usize) -> String {
    let mut a = a;
    let mut b = b;
    let mut res = String::new();
    if a.len() < b.len() {
        mem::swap(&mut a, &mut b);
    }
    for i in 0..n {
        let rrres = char::from_u32(a.chars().nth(i).unwrap_or('0').to_digit(2).unwrap_or(0)
            ^ b.chars().nth(i).unwrap_or('0').to_digit(2).unwrap_or(0) + 48).unwrap_or('0');
        res.push(rrres);
    }
    // res.push_str(&a[b.len()..]);
    res
}

fn mul(a: &str, b: &str) -> String {
    todo!()
}
