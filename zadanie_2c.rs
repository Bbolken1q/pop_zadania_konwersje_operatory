use std::io;

pub fn zadanie() {
    let mut input:Vec<String> = Vec::with_capacity(3);
    let mut suma: f32 = 0.0;
    for _ in 0..3 {
        let mut temp: String = "".to_string();
        io::stdin().read_line(&mut temp).unwrap();
        input.push(temp);
    }
    
    for i in 0..3 {
        suma += input[i].trim().parse::<f32>().unwrap();
    }

    println!("suma tych liczb to {}, a średnia to {}", suma, suma/3.0);
}
