use crate::{compare, mul, remove_last_digits, sub};
use std::cmp::Ordering;

pub fn barret_reduction(x: &str, p: &str, u: &str) -> String {
    let mut q = String::new();
    q = remove_last_digits(
        mul(remove_last_digits(x, p.len() - 1).as_str(), u).as_str(),
        p.len() + 1,
    );
    let mut r = sub(x, mul(q.as_str(), p).as_str());

    while compare(r.as_str(), p) >= Ordering::Equal {
        r = sub(r.as_str(), p);
    }
    r
}
