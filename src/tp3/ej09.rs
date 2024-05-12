use std::collections::VecDeque;

use super::ej03::Fecha;

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: VecDeque<Mascota>, // BinaryHeap requiere Ord trait (tema no correspondiente a la practica)
    registro_atencion: Vec<Atencion>,
}

#[derive(Debug)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animales,
    duenio: Duenio,
}

#[derive(Debug)]
enum Animales {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug)]
struct Duenio {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug)]
struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    prox_visita: Option<Fecha>,
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: u32) -> Veterinaria {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola_atencion: VecDeque::new(),
            registro_atencion: Vec::new(),
        }
    }

    fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    fn agregar_mascota_prioritaria(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    fn atender_mascota(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    fn retirar_mascota(&mut self, mascota: &Mascota) -> bool {
        match self.cola_atencion.iter().position(|m| m.eq(mascota)) {
            Some(index) => {
                self.cola_atencion.remove(index);
                return true;
            }
            None => (),
        }

        false
    }

    fn registrar_atencion(&mut self, atencion: Atencion) {
        self.registro_atencion.push(atencion);
    }

    fn buscar_atencion(
        &self,
        nombre_mascota: &String,
        nombre_duenio: &String,
        telefono: &String,
    ) -> Option<&Atencion> {
        match self
            .registro_atencion
            .iter()
            .position(|at| at.comparar_atencion(nombre_mascota, nombre_duenio, telefono))
        {
            Some(index) => return self.registro_atencion.get(index),
            None => (),
        };

        None
    }

    fn get_pos_atencion(&self, atencion: &Atencion) -> Option<usize> {
        match self.registro_atencion.iter().position(|at| at.eq(atencion)) {
            Some(index) => return Some(index),
            None => (),
        };

        None
    }

    fn modificar_diagnostico(&mut self, atencion: &Atencion, nuevo_diagnostico: String) -> bool {
        match self.get_pos_atencion(atencion) {
            Some(index) => {
                self.registro_atencion
                    .get_mut(index)
                    .unwrap()
                    .set_diagnostico(nuevo_diagnostico);
                return true;
            }
            None => false,
        }
    }

    fn modificar_fecha_visita(&mut self, atencion: &Atencion, nueva_fecha: Option<Fecha>) -> bool {
        match self.get_pos_atencion(atencion) {
            Some(index) => {
                self.registro_atencion
                    .get_mut(index)
                    .unwrap()
                    .set_prox_fecha(nueva_fecha);
                return true;
            }
            None => false,
        }
    }

    fn eliminar_atencion(&mut self, atencion: &Atencion) -> bool {
        match self.get_pos_atencion(atencion) {
            Some(index) => {
                self.registro_atencion.remove(index);
                return true;
            }
            None => (),
        }

        false
    }
}

impl Mascota {
    fn new(nombre: String, edad: u32, tipo: Animales, duenio: Duenio) -> Mascota {
        Mascota {
            nombre,
            edad,
            tipo,
            duenio,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Duenio {
    fn new(nombre: String, direccion: String, telefono: String) -> Duenio {
        Duenio {
            nombre,
            direccion,
            telefono,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Atencion {
    fn new(
        mascota: Mascota,
        diagnostico: String,
        tratamiento: String,
        prox_visita: Option<Fecha>,
    ) -> Atencion {
        Atencion {
            mascota,
            diagnostico,
            tratamiento,
            prox_visita: if let Some(ref f) = prox_visita {
                match f.es_fecha_valida() {
                    true => prox_visita,
                    false => panic!("Fecha no valida"),
                }
            } else {
                None
            },
        }
    }

    fn comparar_atencion(
        &self,
        nombre_mascota: &String,
        nombre_duenio: &String,
        telefono: &String,
    ) -> bool {
        self.mascota.nombre.eq(nombre_mascota)
            && self.mascota.duenio.nombre.eq(nombre_duenio)
            && self.mascota.duenio.telefono.eq(telefono)
    }

    fn set_diagnostico(&mut self, diagnostico: String) {
        self.diagnostico = diagnostico;
    }

    fn set_prox_fecha(&mut self, fecha: Option<Fecha>) {
        match fecha {
            Some(ref f) => match f.es_fecha_valida() {
                true => self.prox_visita = fecha,
                false => panic!("Fecha no valida"),
            },
            None => self.prox_visita = fecha,
        };
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

#[test]
fn test_veterinaria_vacia() {
    let mut veterinaria = Veterinaria::new("Veterinaria".to_string(), "Direccion".to_string(), 5);
    let m1 = Mascota::new(
        "Mascota".to_string(),
        10,
        Animales::Perro,
        Duenio::new(
            "Duenio".to_string(),
            "Direccion duenio".to_string(),
            "Telefono".to_string(),
        ),
    );

    let at1 = Atencion::new(
        m1,
        "Diagnostico".to_string(),
        "Tratamiento".to_string(),
        None,
    );

    assert!(veterinaria.atender_mascota().is_none());

    assert!(veterinaria
        .buscar_atencion(
            &"Mascota".to_string(),
            &"Duenio".to_string(),
            &"Telefono".to_string(),
        )
        .is_none());

    assert!(!veterinaria.modificar_diagnostico(&at1, "Nuevo Diagnostico".to_string()));
    assert!(!veterinaria.modificar_fecha_visita(&at1, Some(Fecha::new(5, 4, 2008))));
    assert!(!veterinaria.eliminar_atencion(&at1));
}

#[test]
fn test_veterinaria_registrar_mascotas() {
    let mut veterinaria = Veterinaria::new("Veterinaria".to_string(), "Direccion".to_string(), 5);

    let m2 = Mascota::new(
        "Mascota2".to_string(),
        15,
        Animales::Gato,
        Duenio::new(
            "Duenio2".to_string(),
            "Direccion duenio2".to_string(),
            "Telefono2".to_string(),
        ),
    );

    let mut v = vec![
        Mascota::new(
            "Mascota1".to_string(),
            10,
            Animales::Perro,
            Duenio::new(
                "Duenio1".to_string(),
                "Direccion duenio1".to_string(),
                "Telefono1".to_string(),
            ),
        ),
        Mascota::new(
            "Mascota2".to_string(),
            15,
            Animales::Gato,
            Duenio::new(
                "Duenio2".to_string(),
                "Direccion duenio2".to_string(),
                "Telefono2".to_string(),
            ),
        ),
        Mascota::new(
            "Mascota3".to_string(),
            4,
            Animales::Otros,
            Duenio::new(
                "Duenio3".to_string(),
                "Direccion duenio3".to_string(),
                "Telefono3".to_string(),
            ),
        ),
    ];

    veterinaria.agregar_mascota_prioritaria(v.pop().unwrap());
    veterinaria.agregar_mascota(v.pop().unwrap());
    veterinaria.agregar_mascota(v.pop().unwrap());

    // Comprubeba cola de atencion y que se haya otorgado la prioridad
    assert_eq!(veterinaria.cola_atencion.len(), 3);
    assert_eq!(
        veterinaria.cola_atencion.front().unwrap().nombre,
        "Mascota3"
    );

    assert_eq!(veterinaria.atender_mascota().unwrap().nombre, "Mascota3");
    assert!(veterinaria.retirar_mascota(&m2));
}

#[test]
fn test_registrar_atenciones() {
    let mut veterinaria = Veterinaria::new("Veterinaria".to_string(), "Direccion".to_string(), 5);

    let m3 = Mascota::new(
        "Mascota3".to_string(),
        4,
        Animales::Otros,
        Duenio::new(
            "Duenio3".to_string(),
            "Direccion duenio3".to_string(),
            "Telefono3".to_string(),
        ),
    );

    let m3_copy = Mascota::new(
        "Mascota3".to_string(),
        4,
        Animales::Otros,
        Duenio::new(
            "Duenio3".to_string(),
            "Direccion duenio3".to_string(),
            "Telefono3".to_string(),
        ),
    );

    let mut at1 = Atencion::new(
        m3,
        "Diagnostico".to_string(),
        "Tratamiento".to_string(),
        Some(Fecha::new(29, 2, 2020)),
    );

    veterinaria.registrar_atencion(Atencion::new(
        m3_copy,
        "Diagnostico".to_string(),
        "Tratamiento".to_string(),
        Some(Fecha::new(29, 2, 2020)),
    ));

    assert!(veterinaria
        .registro_atencion
        .first()
        .unwrap()
        .prox_visita
        .as_ref()
        .is_some_and(|f| f.eq(&Fecha::new(29, 2, 2020))));

    assert!(veterinaria
        .buscar_atencion(
            &"Mascota3".to_string(),
            &"Duenio3".to_string(),
            &"Telefono3".to_string()
        )
        .is_some_and(|f| f.eq(&at1)));

    assert!(veterinaria.modificar_fecha_visita(&at1, Some(Fecha::new(1, 2, 2000))));

    at1.set_prox_fecha(Some(Fecha::new(1, 2, 2000)));

    assert!(veterinaria.modificar_diagnostico(&at1, "Nuevo diagnostico".to_string()));
    assert_eq!(
        veterinaria.registro_atencion.first().unwrap().diagnostico,
        "Nuevo diagnostico"
    );

    assert!(!veterinaria.eliminar_atencion(&at1));
    at1.set_diagnostico("Nuevo diagnostico".to_string());
    assert!(veterinaria.eliminar_atencion(&at1));
}
