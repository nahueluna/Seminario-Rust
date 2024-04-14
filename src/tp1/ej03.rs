pub fn ej3() {
    let bool_op1: bool = true;
    let bool_op2: bool;

    let mut input: String = String::new();

    println!("Ingrese un valor booleano: ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Error al leer");
    bool_op2 = input.trim().parse().expect("Valor no booleano");

    println!(
        "Operacion and = {}, operacion or = {}",
        (bool_op1 && bool_op2),
        (bool_op1 || bool_op2)
    );
}
