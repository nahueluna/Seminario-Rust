pub fn ej5() {
    let cadena: String = String::from("Hello ");
    let mut input: String = String::new();

    println!("Ingrese una cadena: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error de lectura");

    println!("Cadena concatenada: {}", (cadena + &input).to_uppercase());
}
