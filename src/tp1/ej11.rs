pub fn ej11() {
    let array = ["this", "is", "a", "string", "array"];

    let mut input: String = String::new();

    println!("Ingrese una cadena: ");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error de lectura");

    let sub_string = input.trim();

    if array.contains(&sub_string) {
        println!("La cadena ingresada se encuentra en el arreglo");
    }
}
