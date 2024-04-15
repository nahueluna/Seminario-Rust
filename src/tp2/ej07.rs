pub fn ej7() {
    let arreglo = [5, 8, 2, 20, 4];
    let limite = 7;

    println!(
        "Cantidad de mayores a {}: {}",
        limite,
        cantidad_de_mayores(&arreglo, limite)
    );
}

fn cantidad_de_mayores(arreglo: &[i32], limite: i32) -> i32 {
    let mut cant_mayor = 0;
    arreglo.iter().for_each(|n| {
        if *n > limite {
            cant_mayor += 1
        }
    });

    cant_mayor
}

#[test]
fn test_cantidad_mayores() {
    let arreglo = [5, 8, 2, 20, 4];
    let limite = 7;

    let resul = cantidad_de_mayores(&arreglo, limite);

    assert_eq!(resul, 2);
}
