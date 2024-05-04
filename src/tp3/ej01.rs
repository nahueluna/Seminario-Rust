#[derive(Debug)]
struct Persona {
    nombre: String,
    edad: u32,
    direccion: Option<String>,
}

impl Persona {
    fn new(nombre: String, edad: u32, direccion: Option<String>) -> Persona {
        Persona {
            nombre,
            edad,
            direccion,
        }
    }

    fn to_string(&self) -> String {
        //se requiere un string propio en la izquierda. Para no perder propiedad lo clono
        let mut aux = String::from(self.nombre.clone() + "\n" + &self.edad.to_string() + "\n");

        match &self.direccion {
            Some(dir) => aux.push_str(dir),
            None => (),
        }

        aux
    }

    fn obtener_edad(&self) -> u32 {
        self.edad
    }

    fn actualizar_direccion(&mut self, direccion: Option<String>) {
        self.direccion = direccion;
    }
}

#[test]
fn test_construccion_y_metodos() {
    let p1 = Persona::new("Nahuel".to_string(), 20, Some("La Plata".to_string()));
    let mut p2 = Persona::new("Pedro".to_string(), 35, None);

    assert_eq!(p1.obtener_edad(), 20);
    assert_eq!(p1.to_string(), "Nahuel\n20\nLa Plata");

    p2.actualizar_direccion(Some("Ensenada".to_string()));

    assert_eq!(p2.to_string(), "Pedro\n35\nEnsenada");
}

#[test]
fn test_conservar_propiedad() {
    let p1 = Persona::new("Nahuel".to_string(), 20, Some("La Plata".to_string()));
    let mut p2 = Persona::new("Pedro".to_string(), 35, None);

    p1.to_string();
    assert_eq!(p1.direccion, Some("La Plata".to_string()));

    assert_eq!(p2.direccion, None);

    p2.actualizar_direccion(Some("Ensenada".to_string()));

    assert_eq!(p2.direccion, Some("Ensenada".to_string()));
}
