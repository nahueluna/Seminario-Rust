pub fn ej12() {
    let mut enteros = [1, 2, 3, 4, 5];

    println!("Original array: {:?}", enteros);

    reemplazar_pares(&mut enteros);

    println!("Modified array: {:?}", enteros);
}

fn reemplazar_pares(enteros: &mut [i32]) {
    for n in enteros {
        if *n % 2 == 0 {
            *n = -1;
        }
    }
}

#[test]
fn test_reemplazar_pares() {
    let mut enteros = [1, 2, 3, 4, 5];
    reemplazar_pares(&mut enteros);

    assert_eq!(enteros, [1, -1, 3, -1, 5]);
}

#[test]
fn test_reemplazar_pares_todos_pares() {
    let mut enteros = [2, 4, 6, 8];
    reemplazar_pares(&mut enteros);

    assert_eq!(enteros, [-1, -1, -1, -1]);
}

#[test]
fn test_reemplazar_pares_todos_impares() {
    let mut enteros = [1, 3, 5, 7];
    reemplazar_pares(&mut enteros);

    assert_eq!(enteros, [1, 3, 5, 7]);
}
