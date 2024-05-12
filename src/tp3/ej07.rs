struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad: u32,
    autos: Vec<Auto>,
}

#[derive(Debug)]
struct Auto {
    marca: String,
    modelo: String,
    anio: u32,
    precio_bruto: f64,
    color: Colores,
}

#[derive(Debug)]
enum Colores {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

impl Colores {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl ConcesionarioAuto {
    fn new(nombre: String, direccion: String, capacidad: u32) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad,
            autos: Vec::with_capacity(capacidad as usize), // Vec con una capacidad alocada determinada
        }
    }

    fn agregar_auto(&mut self, auto: Auto) -> bool {
        if self.autos.len() < self.capacidad as usize {
            self.autos.push(auto);
            return true;
        }

        false
    }

    fn eliminar_auto(&mut self, auto: &Auto) {
        if let Some(index) = self.autos.iter().position(|a| a.eq(auto)) {
            self.autos.remove(index);
        }
    }

    fn buscar_auto(&self, auto: &Auto) -> Option<&Auto> {
        if let Some(index) = self.autos.iter().position(|a| a.eq(auto)) {
            Some(self.autos.get(index).unwrap()) // Estoy seguro de que está en esa posición
        } else {
            None
        }
    }
}

impl Auto {
    fn new(marca: String, modelo: String, anio: u32, precio_bruto: f64, color: Colores) -> Auto {
        Auto {
            marca,
            modelo,
            anio,
            precio_bruto,
            color,
        }
    }

    fn calcular_precio(&self) -> f64 {
        let mut precio_adicional = match &self.color {
            Colores::Rojo => self.precio_bruto * 1.25,
            Colores::Amarillo => self.precio_bruto * 1.25,
            Colores::Azul => self.precio_bruto * 1.25,
            _ => self.precio_bruto - (self.precio_bruto * 0.1),
        };

        precio_adicional += if self.marca.eq("BMW") {
            self.precio_bruto * 0.15
        } else {
            0.0
        };

        precio_adicional -= if self.anio < 2000 {
            self.precio_bruto * 0.05
        } else {
            0.0
        };

        precio_adicional
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

#[test]
fn test_auto1() {
    let a = Auto::new(
        "BMW".to_string(),
        "Modelo".to_string(),
        1997,
        1000.0,
        Colores::Rojo,
    );

    assert_eq!(a.calcular_precio(), 1350.0);

    assert_eq!(a.marca, "BMW");
    assert_eq!(a.modelo, "Modelo");
    assert!(a.color.eq(&Colores::Rojo));
}

#[test]
fn test_auto2() {
    let a = Auto::new(
        "Marca".to_string(),
        "Modelo".to_string(),
        2005,
        1000.0,
        Colores::Negro,
    );

    assert_eq!(a.calcular_precio(), 900.0);

    assert_eq!(a.marca, "Marca");
    assert_eq!(a.modelo, "Modelo");
    assert!(a.color.eq(&Colores::Negro));
}

#[test]
fn test_concesionario1() {
    // Crea vec autos
    let autos = vec![
        Auto::new(
            "Marca1".to_string(),
            "Modelo1".to_string(),
            1988,
            1000.0,
            Colores::Amarillo,
        ),
        Auto::new(
            "Marca2".to_string(),
            "Modelo2".to_string(),
            2020,
            8764.75,
            Colores::Negro,
        ),
        Auto::new(
            "BMW".to_string(),
            "Modelo3".to_string(),
            1997,
            6432.2,
            Colores::Blanco,
        ),
        Auto::new(
            "Marca3".to_string(),
            "Modelo4".to_string(),
            2008,
            1333.33,
            Colores::Rojo,
        ),
    ];

    let a1 = Auto::new(
        "Marca4".to_string(),
        "Modelo5".to_string(),
        1990,
        100.0,
        Colores::Azul,
    );

    let mut concesionario =
        ConcesionarioAuto::new("Concesionario".to_string(), "Direccion".to_string(), 4);

    // Agrego los autos
    for a in autos {
        concesionario.agregar_auto(a);
    }

    // Intenta agregar auto 1
    assert!(!concesionario.agregar_auto(a1));

    let a2 = Auto::new(
        "Marca3".to_string(),
        "Modelo4".to_string(),
        2008,
        1333.33,
        Colores::Rojo,
    );

    // Longitud de vec autos antes de eliminar
    assert_eq!(concesionario.autos.len(), 4);

    concesionario.eliminar_auto(&a2);

    // Longitud de vec autos despues de eliminar
    assert_eq!(concesionario.autos.len(), 3);

    // Busqueda de auto ya eliminado
    assert!(concesionario.buscar_auto(&a2).is_none());

    let a3 = Auto::new(
        "Marca2".to_string(),
        "Modelo2".to_string(),
        2020,
        8764.75,
        Colores::Negro,
    );

    // Busqueda de auto existente
    assert!(concesionario.buscar_auto(&a3).is_some_and(|a| a.eq(&a3)));

    // Chequea que vec no haya perdido ownership de sus autos
    let precio_auto = concesionario.autos.first().unwrap().calcular_precio();
    assert_eq!(precio_auto, 1200.0);
}

#[test]
fn test_concesionario2() {
    let mut concesionario =
        ConcesionarioAuto::new("Concesionario".to_string(), "Direccion".to_string(), 1);

    let a = Auto::new(
        "Marca".to_string(),
        "Modelo".to_string(),
        1999,
        1000.0,
        Colores::Negro,
    );

    assert!(concesionario.buscar_auto(&a).is_none());

    concesionario.eliminar_auto(&a);

    assert_eq!(concesionario.autos.len(), 0);

    concesionario.agregar_auto(a);

    assert_eq!(concesionario.autos.len(), 1);

    assert_eq!(concesionario.autos.first().unwrap().modelo, "Modelo");
}
