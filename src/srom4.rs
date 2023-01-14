use std::mem;

pub fn add(a: &str, b: &str) -> String {
    let mut a = a;
    let mut b = b;
    let mut res = String::new();

    for i in 0..233 {
        let rrres;
        if a.chars().nth(i).is_none() && b.chars().nth(i).is_none() {
            rrres = '0';
        } else {
            rrres = char::from_u32(a.chars().nth(i).unwrap_or('0').to_digit(2).unwrap_or(0)
                ^ b.chars().nth(i).unwrap_or('0').to_digit(2).unwrap_or(0) + 48).unwrap_or('0');
        }
        res.push(rrres);
    }
    res
}

pub fn mul(a: &str, b: &str) -> String {
    let mut a = a.to_string();
    let mut b = b.to_string();
    let mut res = String::new();
    let mut v: Vec<Vec<i32>> = Vec::new();
    let p = 2 * 233 + 1;
    for i in 0..233 {
        v.push(Vec::new());
        for o in 0..233 {
            if (2i32.pow(i) - 2i32.pow(o)) % p != 1 &&
                (-2i32.pow(i) - 2i32.pow(o)) % p != 1 &&
                (2i32.pow(o) - 2i32.pow(i)) % p != 1 &&
                (2i32.pow(i) + 2i32.pow(o)) % p != 1 {
                v.get_mut(i as usize).unwrap().push(0);
            } else {
                v.get_mut(i as usize).unwrap().push(1);
            }
        }
    }
    for _ in 0..233 {
        let mut new_vec = Vec::new();
        for i in 0..233 {
            let mut m = 0;
            for j in 0..233 {
                m = (m + a.chars().nth(j).unwrap().to_digit(2).unwrap() as i32 *
                    v.get(j).unwrap().get(i).unwrap() ) % 2;
            }
            new_vec.push(m);
        }
        let mut m = 0;
        (0..233).for_each(|ii| m = (m +
            new_vec.get(ii).unwrap() * b.chars().nth(ii).unwrap().to_digit(2).unwrap() as i32 ) % 2);
        res.push_str(m.to_string().as_str());
        a = shift(a.as_str());
        b = shift(b.as_str());
    }
    res
}

pub fn shift(s: &str) -> String {
    let mut ss = String::from(&s[1..]);
    ss.push(s.chars().nth(0).unwrap());
    ss
}
