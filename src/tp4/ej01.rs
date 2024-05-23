fn determinar_cantidad_primos<T>(numeros: &Vec<T>) -> i32
where
    T: Primo,
{
    let mut cant_primos = 0;
    numeros.iter().for_each(|n| {
        if n.es_primo() {
            cant_primos += 1;
        }
    });

    cant_primos
}

pub trait Primo {
    fn es_primo(&self) -> bool;
}

impl Primo for i32 {
    fn es_primo(&self) -> bool {
        if self <= &1 {
            return false;
        }

        for n in 2..(self / 2) + 1 {
            if self % n == 0 {
                return false;
            }
        }

        true
    }
}

#[test]
fn test_vector_vacio() {
    let v: Vec<i32> = Vec::new();

    assert_eq!(determinar_cantidad_primos(&v), 0);
}

#[test]
fn test_ningun_primo() {
    let v = vec![4, 8, 9, 10, 12, 14];

    assert_eq!(determinar_cantidad_primos(&v), 0);
}

#[test]
fn test_todos_primos() {
    let v = vec![2, 3, 5, 7, 11];

    assert_eq!(determinar_cantidad_primos(&v), v.len() as i32);
}

#[test]
fn test_primos_y_no_primos() {
    let v = vec![1, 3, 4, 600, 577];

    assert_eq!(determinar_cantidad_primos(&v), 2);
}
