pub fn ej5() {
    let array = [1.1, 2.2, 3.3, 4.4, 5.5];

    println!("Array duplicado: {:?}", duplicar_valores(array));
}

fn duplicar_valores(flotantes: [f64; 5]) -> [f64; 5] {
    let mut resultado = flotantes;
    resultado.iter_mut().for_each(|n| *n *= 2 as f64);

    resultado
}

#[test]
fn test_duplicar_valores() {
    let array_original = [1.1, 2.2, 3.3, 4.4, 5.5];
    let array_duplicados = [2.2, 4.4, 6.6, 8.8, 11.0];

    let resultado = duplicar_valores(array_original);

    assert_eq!(resultado, array_duplicados);
}
