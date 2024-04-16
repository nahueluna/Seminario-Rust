pub fn ej8() {
    let array = [1.1, 2.2, 3.3, 4.4, 5.5];

    let resultado = suma_arreglos(array, array);

    println!("Nuevo arreglo: {:?}", resultado);
}

fn suma_arreglos(arr1: [f64; 5], arr2: [f64; 5]) -> [f64; 5] {
    let mut resul = [0.0; 5];

    for i in 0..arr1.len() {
        resul[i] = arr1[i] + arr2[i];
    }

    resul
}

#[test]
fn test_suma_arreglos() {
    let array = [1.1, 2.2, 3.3, 4.4, 5.5];
    let resultado = suma_arreglos(array, array);
    assert_eq!(resultado, [2.2, 4.4, 6.6, 8.8, 11.0]);
}
