pub fn ej7() {
    const CONSTANTE: i32 = 3;
    let mut array = [5, 4, 7, 10, 8, 1];

    println!("{:?}", array);

    /* Un closure es una función anónima la cual ejecuta determinado
    código y puede tener parámetros. Se define estableciendo "| |" en su inicio
    En este caso se define "|n|" que toma cada valor del array.
    Con "*n" es desreferenciado y luego modificado. */
    array.iter_mut().for_each(|n| *n *= CONSTANTE);

    println!("{:?}", array);
}
