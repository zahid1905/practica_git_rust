use std::io;

fn main() {
    let mut input: String = String::new();

    println!("Ingrese un valor double: ");

    io::stdin().read_line(&mut input).expect("No se pudo leer");

    let valor_double: f64 = input.trim().parse().unwrap();

    println!("Valor double: {}", valor_double);
}
