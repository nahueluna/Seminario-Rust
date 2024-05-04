#[derive(Debug)]
struct Rectangulo {
    longitud: u32,
    ancho: u32,
}

impl Rectangulo {
    fn new(longitud: u32, ancho: u32) -> Rectangulo {
        Rectangulo { longitud, ancho }
    }

    fn calcular_area(&self) -> u32 {
        self.longitud * self.ancho
    }

    fn calcular_perimetro(&self) -> u32 {
        2 * (self.longitud + self.ancho)
    }

    fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

#[test]
fn test_rectangulo() {
    let r = Rectangulo::new(10, 5);

    assert_eq!(r.calcular_area(), 50);
    assert_eq!(r.calcular_perimetro(), 30);
    assert!(!r.es_cuadrado());
}

#[test]
fn test_cuadrado() {
    let c = Rectangulo::new(3, 3);

    assert_eq!(c.calcular_area(), 9);
    assert_eq!(c.calcular_perimetro(), 12);
    assert!(c.es_cuadrado());
}

#[test]
fn test_lados_nulos() {
    let r = Rectangulo::new(0, 0);

    assert_eq!(r.calcular_area(), 0);
    assert_eq!(r.calcular_perimetro(), 0);
    assert!(r.es_cuadrado());
}
