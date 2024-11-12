use std::io;

pub fn zadanie() {
    let mut input:Vec<String> = Vec::with_capacity(2);
    let mut suma: f32 = 1.0;
    for _ in 0..2 {
        let mut temp: String = "".to_string();
        io::stdin().read_line(&mut temp).unwrap();
        input.push(temp);
    }
    
    for i in 0..2 {
        suma *= input[i].trim().parse::<f32>().unwrap();
    }

    println!("pole tego prostokąta to {}", suma);
}