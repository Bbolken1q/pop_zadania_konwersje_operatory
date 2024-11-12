use std::{any::type_name, io};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn zadanie() {
    let mut input = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut input).unwrap();
    io::stdin().read_line(&mut input2).unwrap();
    let l1: i32 = input.trim().parse::<i32>().unwrap();
    let l2: i32 = input2.trim().parse::<i32>().unwrap();

    println!("{}", l1 % l2);
}
