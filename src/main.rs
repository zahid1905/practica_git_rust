use std::io;

fn main() {
    let mut input: String = String::new();

    println!("Ingrese un valor: ");

    io::stdin().read_line(&mut input).expect("No se pudo leer");

    let valor_int: i32 = input.trim().parse().unwrap();
    let valor_byte: u8 = input.trim().parse().unwrap();
    let valor_double: f64 = input.trim().parse().unwrap();
    
    println!("Valor int: {}", valor_int);
    println!("Valor byte: {}", valor_byte);
    println!("Valor double: {}", valor_double);
}
