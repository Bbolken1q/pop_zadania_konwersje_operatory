use std::{any::type_name, io};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

pub fn zadanie() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let int: i32 = input.trim().parse::<i32>().unwrap();
    println!("{}, {}", int, type_of(int));
    let string: String = int.to_string();
    println!("{}, {}", string, type_of(string.clone()));
    let int2: i32 = input.trim().parse::<i32>().unwrap();
    println!("{}, {}", int2, type_of(int2));
}
