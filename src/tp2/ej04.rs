pub fn ej4() {
    let array = [1, 2, 3, 4, 5];
    println!("Cantidad de impares: {}", cantidad_impares(&array));
}

fn cantidad_impares(numeros: &[i32]) -> i32 {
    let mut cant_impares = 0;

    for n in numeros {
        if n % 2 != 0 {
            cant_impares += 1;
        }
    }

    cant_impares
}

#[test]
fn test_cantidad_impares() {
    let array = [1, 2, 3, 4, 5];
    let resul = cantidad_impares(&array);

    assert_eq!(resul, 3);
}
