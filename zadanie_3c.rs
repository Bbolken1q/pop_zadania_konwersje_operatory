use std::io;

pub fn zadanie() {
    let mut temp: String = "".to_string();
    io::stdin().read_line(&mut temp).unwrap();
    let liczba = temp.trim().parse::<f64>().unwrap();

    drop(temp);

    if liczba != 10.0 {
        println!("Liczba nie równa się 10");
    }
}