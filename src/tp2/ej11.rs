pub fn ej11() {
    const FACTOR: i32 = 4;

    let mut array = [2, 4, 6, 8];

    println!("Original array: {:?}", array);

    multiplicar_valores(&mut array, FACTOR);

    println!("Multiplied array: {:?}", array);
}

fn multiplicar_valores(arreglo: &mut [i32], factor: i32) {
    arreglo.iter_mut().for_each(|x| *x *= factor);
}

#[test]
fn test_multiplicar_valores() {
    let mut array = [2, 4, 6, 8];

    multiplicar_valores(&mut array, 4);

    assert_eq!(array[2], 24);
}

#[test]
fn test_multiplicar_por_cero() {
    let mut array = [2, 4, 6, 8];

    multiplicar_valores(&mut array, 0);

    assert_eq!(array, [0, 0, 0, 0]);
}

#[test]
fn test_multiplicar_por_negativo() {
    let mut array = [2, 4, 6, 8];

    multiplicar_valores(&mut array, -1);

    assert_eq!(array, [-2, -4, -6, -8]);
}
