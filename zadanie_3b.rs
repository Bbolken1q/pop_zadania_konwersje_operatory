use std::io;

pub fn zadanie() {
    let mut temp: String = "".to_string();
    io::stdin().read_line(&mut temp).unwrap();
    let liczba = temp.trim().parse::<f64>().unwrap();

    drop(temp);

    if liczba < 5.0 {
        println!("Liczba mniejsza od 5");
    }
    else if liczba > 15.0 {
        println!("Liczba większa od 15");
    }
}