use std::io;

pub fn zadanie() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let dob: f64 = input.trim().parse::<f64>().unwrap();
    println!("{}", dob);
    println!("{}", dob*dob);
}
