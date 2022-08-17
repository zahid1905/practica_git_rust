use std::io;

fn main() {
    let mut input: String = String::new();

    println!("Ingrese un valor string: ");

    io::stdin().read_line(&mut input).expect("No se pudo leer");

    let valor_string: String = input.trim().parse().unwrap();

    println!("Valor string: {}", valor_string);
}
