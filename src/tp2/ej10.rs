pub fn ej10() {
    const LIMITE: i32 = 4;

    let array_str = ["This", "is", "a", "String", "array"];
    let mut cadenas: [String; 5] = Default::default();

    for (i, c) in array_str.iter().enumerate() {
        cadenas[i] = c.to_string();
    }

    let resul = cantidad_de_cadenas_mayor_a(&cadenas, LIMITE);
    println!(
        "Cantidad de cadenas con longitud mayor a {}: {}",
        LIMITE, resul
    );
}

fn cantidad_de_cadenas_mayor_a(cadenas: &[String], limite: i32) -> i32 {
    let mut cant_mayor_long = 0;

    cadenas.iter().for_each(|n| match n {
        x if (x.len() as i32) > limite => cant_mayor_long += 1,
        _ => (),
    });

    cant_mayor_long
}

#[test]
fn test_contar_longitud_cadena() {
    let array_str = ["This", "is", "a", "String", "array"];
    let mut cadenas: [String; 5] = Default::default();

    for (i, c) in array_str.iter().enumerate() {
        cadenas[i] = c.to_string();
    }

    let resul = cantidad_de_cadenas_mayor_a(&cadenas, 4);

    assert_eq!(resul, 2);
}
