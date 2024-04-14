pub fn ej12() {
    let tupla = ("Hello World!", [1, 2, 3]);

    println!(
        "Cadena: {}, valores enteros sumados: {}",
        tupla.0,
        tupla.1.iter().sum::<i32>()
    )
}
