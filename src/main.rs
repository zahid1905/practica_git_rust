use std::io;

fn main() {
    let mut input: String = String::new();

    println!("Ingreseun valor int: ");

    io::stdin().read_line(&mut input).expect("No se pudo leer");

    let valor_int: i32 = input.trim().parse().unwrap();

    println!("Valor int: {}", valor_int);
}
