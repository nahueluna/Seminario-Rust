pub fn ej5() {
    let number: u32 = 27;
    let mut input: String = String::new();

    println!("Ingrese un valor numerico: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer");

    let operator: u32 = input.trim().parse().expect("Valor no numérico");

    let result = number + operator;

    println!("({} + {})² = {}", number, operator, result.pow(2));
}
