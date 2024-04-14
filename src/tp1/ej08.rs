pub fn ej8() {
    const CADENA: &str = "Hello World!";

    let mut input: String = String::new();

    println!("Ingrese un caracter: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error de lectura");

    let c: char = input.trim().parse().expect("Valor no char");
    let mut cant_char: u16 = 0;

    for i in CADENA.chars() {
        if i == c {
            cant_char += 1;
        };
    }

    println!(
        "El caracter '{}' se encuentra {} veces en la cadena '{}'",
        c, cant_char, CADENA
    );
}
