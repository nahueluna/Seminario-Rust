pub fn ej2() {
    let n = 8;
    println!("{} es primo? {}", n, es_primo(n));
}

fn es_primo(num: u32) -> bool {
    if num <= 1 {
        return false;
    }

    for n in 2..(num / 2) {
        if num % n == 0 {
            return false;
        }
    }

    true
}

#[test]
fn test_numero_primo() {
    let numero = 4;

    assert!(es_primo(numero));
}
