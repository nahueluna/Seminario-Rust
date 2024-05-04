struct Producto {
    nombre: String,
    precio_bruto: f64,
    id: i32,
}

impl Producto {
    fn new(nombre: String, precio_bruto: f64, id: i32) -> Producto {
        Producto {
            nombre,
            precio_bruto,
            id,
        }
    }

    fn calcular_impuestos(&self, porcentaje_impuestos: Option<u32>) -> f64 {
        if let Some(impuestos) = porcentaje_impuestos {
            self.precio_bruto * (impuestos as f64 / 100.0)
        } else {
            0.0
        }
    }

    fn aplicar_descuento(&self, porcentaje_descuento: Option<u32>) -> f64 {
        if let Some(descuento) = porcentaje_descuento {
            self.precio_bruto * (descuento as f64 / 100.0)
        } else {
            0.0
        }
    }

    fn calcular_precio_total(
        &self,
        porcentaje_impuestos: Option<u32>,
        porcentaje_descuento: Option<u32>,
    ) -> f64 {
        self.precio_bruto + self.calcular_impuestos(porcentaje_impuestos)
            - self.aplicar_descuento(porcentaje_descuento)
    }
}

#[test]
fn test_producto1() {
    let p = Producto::new("Producto 1".to_string(), 100.0, 5);
    let impuesto = Some(50);
    let descuento = Some(10);

    assert_eq!(p.aplicar_descuento(descuento), 10.0);
    assert_eq!(p.calcular_impuestos(impuesto), 50.0);
    assert_eq!(p.calcular_precio_total(impuesto, descuento), 140.0);
}

#[test]
fn test_producto2() {
    let p = Producto::new("Producto 2".to_string(), 200.5, 5);
    let impuesto = Some(30);
    let descuento = None;

    assert_eq!(p.aplicar_descuento(descuento), 0.0);
    assert_eq!(p.calcular_impuestos(impuesto), 60.15);
    assert_eq!(p.calcular_precio_total(impuesto, descuento), 260.65);
}
