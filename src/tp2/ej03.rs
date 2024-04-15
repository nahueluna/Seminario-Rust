pub fn ej3() {
    let array = [1, 2, 3, 4, 5];

    println!("Suma de pares del arreglo = {}", suma_pares(&array));
}

fn suma_pares(numeros: &[i32]) -> i32 {
    let mut resultado: i32 = 0;

    for n in numeros {
        if n % 2 == 0 {
            resultado += n;
        }
    }

    resultado
}

#[test]
fn test_suma_pares() {
    let array = [2, 4, 6, 8, 10];
    let resul = suma_pares(&array);

    assert_eq!(resul, 30);
}
