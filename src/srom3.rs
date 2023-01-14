use std::mem;

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

fn add_right_zero(a: &str, i: usize) -> String {
    let mut a = String::from(a);
    (0..i).for_each(|_| a.push('0'));
    a
}

fn mul(a: &str, b: &str) -> String {
    todo!()
}
