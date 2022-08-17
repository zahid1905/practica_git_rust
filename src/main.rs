use std::io;

fn main() {
    let mut input: String = String::new();

    println!("Ingrese un valor byte: ");

    io::stdin().read_line(&mut input).expect("No se pudo leer");

    let valor_byte: u8 = input.trim().parse().unwrap();

    println!("Valor int: {}", valor_byte);
}
