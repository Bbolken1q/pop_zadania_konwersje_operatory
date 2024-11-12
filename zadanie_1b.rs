use std::{any::type_name, io};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn zadanie() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let dob: f64 = input.trim().parse::<f64>().unwrap();
    println!("{}, {}", dob, type_of(dob));
    let int: i32 = dob as i32;
    println!("{}, {}", int, type_of(int));
}
