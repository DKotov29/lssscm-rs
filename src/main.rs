use lsscm::Number;

fn main() {
    let a;
    println!("Enter number A in hex");
    // let mut A_str = String::new();
    // std::io::stdin()
    //     .read_line(&mut A_str)
    //     .expect("not valid utf8");
    let mut A_str = String::from("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    a = Number::get_from_str(&A_str).unwrap();
    let b;
    println!("Enter number B in hex");
    // let mut B_str = String::new();
    // std::io::stdin()
    //     .read_line(&mut B_str)
    //     .expect("not valid utf8");
    let mut B_str = String::from("0xFFFFFFFFFFFFFFFF");
    b = Number::get_from_str(&B_str).unwrap();
    println!("{:#x}", &a + &b);
}
