pub fn ej1() {
    let n = 5;
    println!("El numero {} es par? {}", n, es_par(n));
}

fn es_par(num: i32) -> bool {
    return num % 2 == 0;
}

#[test]
fn test_es_par() {
    let n = 4;
    assert!(es_par(n));
}
