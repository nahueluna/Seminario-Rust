use std::default;

pub fn ej13() {
    let array_str = ["Pablo", "Nahuel", "Juan", "Anabela", "Dario"];
    let mut nombres: [String; 5] = Default::default();

    for (i, c) in array_str.iter().enumerate() {
        nombres[i] = c.to_string();
    }

    println!("Original array: {:?}", nombres);

    ordenar_nombres(&mut nombres);

    println!("Modified array: {:?}", nombres);
}

fn ordenar_nombres(nombres: &mut [String]) {
    nombres.sort();
}

#[test]
fn test_ordenar_nombres() {
    let array_str = ["Pablo", "Nahuel", "Juan", "Anabela", "Dario"];
    let mut nombres: [String; 5] = Default::default();

    for (i, c) in array_str.iter().enumerate() {
        nombres[i] = c.to_string();
    }

    ordenar_nombres(&mut nombres);

    assert_eq!(nombres, ["Anabela", "Dario", "Juan", "Nahuel", "Pablo"]);
}

#[test]
fn test_ordenar_nombres_array_vacio() {
    let mut nombres: [String; 5] = Default::default();

    ordenar_nombres(&mut nombres);

    assert_eq!(nombres, ["", "", "", "", ""]);
}
