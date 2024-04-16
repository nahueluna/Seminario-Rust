pub fn ej9() {
    let arreglo = [1, 5, 9, 0, 3, 2];
    let resul = cantidad_en_rango(&arreglo, 3, 7);

    println!("Cantidad de numeros en el rango: {}", resul);
}

fn cantidad_en_rango(array: &[i32], inferior: i32, superior: i32) -> i32 {
    let mut cant_num = 0;

    if inferior > superior {
        return 0;
    }

    for n in array {
        if (*n >= inferior) & (*n <= superior) {
            cant_num += 1;
        }
    }

    cant_num
}

#[test]
fn test_rango() {
    let arreglo = [1, 5, 9, 0, 3, 2];
    let resul = cantidad_en_rango(&arreglo, 3, 7);

    assert_eq!(resul, 2);
}

#[test]
fn test_rango_limites_iguales() {
    let arreglo = [1, 5, 9, 0, 3, 2];
    let resul = cantidad_en_rango(&arreglo, 3, 3);

    assert_eq!(resul, 1);
}

#[test]
fn test_rango_limites_invertidos() {
    let arreglo = [1, 5, 9, 0, 3, 2];
    let resul = cantidad_en_rango(&arreglo, 3, 1);

    assert_eq!(resul, 0);
}
