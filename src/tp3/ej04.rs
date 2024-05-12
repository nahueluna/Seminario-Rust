struct Triangulo {
    lado1: u32,
    lado2: u32,
    lado3: u32,
}
#[derive(Debug)]
enum TipoTriangulo {
    Equilatero,
    Isosceles,
    Escaleno,
}

impl TipoTriangulo {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Triangulo {
    fn new(lado1: u32, lado2: u32, lado3: u32) -> Triangulo {
        Triangulo {
            lado1,
            lado2,
            lado3,
        }
    }

    fn determinar_tipo(&self) -> TipoTriangulo {
        if self.lado1 == self.lado2 {
            if self.lado1 == self.lado3 {
                TipoTriangulo::Equilatero
            } else {
                TipoTriangulo::Isosceles
            }
        } else if self.lado2 == self.lado3 {
            TipoTriangulo::Isosceles
        } else {
            TipoTriangulo::Escaleno
        }
    }

    fn calcular_area(&self) -> f64 {
        let s = self.calcular_perimetro() as f64 / 2.0;

        f64::sqrt(s * (s - self.lado1 as f64) * (s - self.lado2 as f64) * (s - self.lado3 as f64))
    }

    fn calcular_perimetro(&self) -> u32 {
        self.lado1 + self.lado2 + self.lado3
    }
}

#[test]
fn test_tipo_equilatero() {
    let t = Triangulo::new(6, 6, 6);

    assert!(t.determinar_tipo().eq(&TipoTriangulo::Equilatero));

    assert_eq!(t.calcular_perimetro(), 18);
    assert_eq!(t.calcular_area(), 15.588457268119896);
}

#[test]
fn test_tipo_isosceles() {
    let t1 = Triangulo::new(3, 5, 5);
    let t2 = Triangulo::new(4, 4, 8);

    assert!(t1.determinar_tipo().eq(&TipoTriangulo::Isosceles));
    assert!(t2.determinar_tipo().eq(&TipoTriangulo::Isosceles));

    assert_eq!(t1.calcular_perimetro(), 13);
    assert_eq!(t1.calcular_area(), 7.1545440106270926);

    assert_eq!(t2.calcular_perimetro(), 16);
    assert_eq!(t2.calcular_area(), 0.0);
}

#[test]
fn test_tipo_escaleno() {
    let t = Triangulo::new(3, 6, 4);

    assert!(t.determinar_tipo().eq(&TipoTriangulo::Escaleno));

    assert_eq!(t.calcular_perimetro(), 13);
    assert_eq!(t.calcular_area(), 5.332682251925386);
}

#[test]
fn test_lado_0() {
    let t = Triangulo::new(0, 0, 0);

    assert_eq!(t.calcular_area(), 0 as f64);
    assert_eq!(t.calcular_perimetro(), 0);
}
