use std::io;

pub fn zadanie() {
    let mut temp: String = "".to_string();
    io::stdin().read_line(&mut temp).unwrap();
    let liczba = temp.trim().parse::<f64>().unwrap();

    drop(temp);

    if liczba >= 1.0 || liczba <= 100.0  {
        println!("Liczba mieści się w przedziale 1-100");
    }
}