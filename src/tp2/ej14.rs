use std::f64::consts::PI;

pub fn ej14() {
    let mut float = 3.14;

    println!("Original number: {}", float);

    incrementar(&mut float);

    println!("Increased number: {}", float);
}

fn incrementar(num: &mut f64) {
    *num += 1 as f64;
}

#[test]
fn test_incrementar() {
    let mut float = 5.5;
    incrementar(&mut float);

    assert_eq!(float, 6.5);
}

#[test]
fn test_incrementar_pi() {
    let mut float = PI;
    incrementar(&mut float);

    assert_eq!(float, (PI + 1.0));
}

#[test]
fn test_incrementar_negativo() {
    let mut float = -0.5;
    incrementar(&mut float);

    assert_eq!(float, 0.5);
}
