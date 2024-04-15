pub fn ej6() {
    let original_array = ["this", "is", "a", "string", "array"];
    let mut string_array: [String; 5] = Default::default(); //setea valores por defecto

    for (i, c) in original_array.iter().enumerate() {
        string_array[i] = c.to_string();
    }

    println!(
        "La longitud de las cadenas es: {:?}",
        longitud_de_cadenas(&string_array)
    );
}

fn longitud_de_cadenas(cadenas: &[String; 5]) -> [i32; 5] {
    let mut resultado = [0; 5];

    for (index, cad) in cadenas.iter().enumerate() {
        resultado[index] = cad.len() as i32;
    }

    resultado
}

#[test]
fn test_longitud_cadenas() {
    let original_array = ["this", "is", "a", "string", "array"];
    let mut string_array: [String; 5] = Default::default(); //setea valores por defecto

    for (i, c) in original_array.iter().enumerate() {
        string_array[i] = c.to_string();
    }

    let resul = longitud_de_cadenas(&string_array);

    assert_eq!(resul, [4, 2, 1, 6, 5]);
}
