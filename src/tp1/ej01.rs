pub fn ej1() {
    let float: f64 = 3.14;
    let float_operator: f64;

    let mut input: String = String::new();

    println!("Ingrese un valor flotante: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer el numero");

    float_operator = input
        .trim()
        .parse()
        .expect("No se ha ingresado un valor válido");

    println!("Suma: {:.2}", float + float_operator);
    println!("Resta: {:.2}", float - float_operator);
    println!("Multiplicación: {:.2}", float * float_operator);
    println!("División: {:.2}", float / float_operator);
}
