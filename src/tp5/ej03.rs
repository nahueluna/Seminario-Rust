use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::{fmt::Display, fs::File, fs::OpenOptions, io::prelude::*, path::Path};

use crate::tp3::ej03::Fecha;

struct Veterinaria {
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: VecDeque<Mascota>, // BinaryHeap requiere Ord trait (tema no correspondiente a la practica)
    registro_atencion: Vec<Atencion>,
    file_path: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Mascota {
    nombre: String,
    edad: u32,
    tipo: Animales,
    duenio: Duenio,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
enum Animales {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Duenio {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
struct Atencion {
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    prox_visita: Option<Fecha>,
}

impl Veterinaria {
    fn new(nombre: String, direccion: String, id: u32, file_path: String) -> Veterinaria {
        Veterinaria {
            nombre,
            direccion,
            id,
            cola_atencion: VecDeque::new(),
            registro_atencion: Vec::new(),
            file_path,
        }
    }

    fn actualizar_archivo(&self, atenciones: &Vec<Atencion>) -> Result<(), ErrorVeterinaria> {
        match File::create(&self.file_path) {
            Ok(mut f) => {
                let Ok(at) = serde_json::to_string_pretty(&atenciones) else {
                    return Err(ErrorVeterinaria::Archivo);
                };

                let Ok(_) = f.write_all(&at.as_bytes()) else {
                    return Err(ErrorVeterinaria::Archivo);
                };

                Ok(())
            }
            Err(_) => Err(ErrorVeterinaria::Archivo),
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

    fn registrar_atencion(&mut self, atencion: Atencion) -> Result<(), ErrorVeterinaria> {
        self.registro_atencion.push(atencion);
        self.actualizar_archivo(&self.registro_atencion)
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

    fn modificar_diagnostico(
        &mut self,
        atencion: &Atencion,
        nuevo_diagnostico: String,
    ) -> Result<(), ErrorVeterinaria> {
        match self.get_pos_atencion(atencion) {
            Some(index) => {
                self.registro_atencion
                    .get_mut(index)
                    .unwrap()
                    .set_diagnostico(nuevo_diagnostico);

                self.actualizar_archivo(&self.registro_atencion)
            }
            None => Err(ErrorVeterinaria::ModificarAtencion),
        }
    }

    fn modificar_fecha_visita(
        &mut self,
        atencion: &Atencion,
        nueva_fecha: Option<Fecha>,
    ) -> Result<(), ErrorVeterinaria> {
        match self.get_pos_atencion(atencion) {
            Some(index) => {
                self.registro_atencion
                    .get_mut(index)
                    .unwrap()
                    .set_prox_fecha(nueva_fecha);

                self.actualizar_archivo(&self.registro_atencion)
            }
            None => Err(ErrorVeterinaria::ModificarAtencion),
        }
    }

    fn eliminar_atencion(&mut self, atencion: &Atencion) -> Result<(), ErrorVeterinaria> {
        match self.get_pos_atencion(atencion) {
            Some(index) => {
                self.registro_atencion.remove(index);

                self.actualizar_archivo(&self.registro_atencion)
            }
            None => Err(ErrorVeterinaria::EliminarAtencion),
        }
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
}

impl Duenio {
    fn new(nombre: String, direccion: String, telefono: String) -> Duenio {
        Duenio {
            nombre,
            direccion,
            telefono,
        }
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
}

#[derive(Debug, PartialEq)]
enum ErrorVeterinaria {
    Archivo,
    EliminarAtencion,
    ModificarAtencion,
}

impl Display for ErrorVeterinaria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorVeterinaria::Archivo => write!(
                f,
                "Ha ocurrido un error mientras se trabajaba con el archivo"
            ),
            ErrorVeterinaria::EliminarAtencion => write!(f, "La atencion no se ha podido eliminar"),
            ErrorVeterinaria::ModificarAtencion => {
                write!(f, "La atencion no se ha podido modificar")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn creacion_veterinaria() -> Veterinaria {
        let mut veterinaria = Veterinaria::new(
            "Veterinaria".to_string(),
            "Direccion".to_string(),
            5,
            Default::default(),
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

        veterinaria
    }

    #[test]
    fn test_veterinaria_vacia() {
        let mut veterinaria = Veterinaria::new(
            "Veterinaria".to_string(),
            "Direccion".to_string(),
            5,
            Default::default(),
        );
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

        assert_eq!(
            veterinaria.modificar_diagnostico(&at1, "Nuevo Diagnostico".to_string()),
            Err(ErrorVeterinaria::ModificarAtencion)
        );
        assert_eq!(
            veterinaria.modificar_fecha_visita(&at1, Some(Fecha::new(5, 4, 2008))),
            Err(ErrorVeterinaria::ModificarAtencion)
        );
        assert_eq!(
            veterinaria.eliminar_atencion(&at1),
            Err(ErrorVeterinaria::EliminarAtencion)
        );
    }

    #[test]
    fn test_veterinaria_registrar_mascotas() {
        let mut veterinaria = Veterinaria::new(
            "Veterinaria".to_string(),
            "Direccion".to_string(),
            5,
            Default::default(),
        );

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

    #[should_panic]
    #[test]
    fn test_registrar_atenciones() {
        let mut veterinaria = Veterinaria::new(
            "Veterinaria".to_string(),
            "Direccion".to_string(),
            5,
            "test_files/veterinaria1.json".to_string(),
        );

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

        veterinaria.modificar_fecha_visita(&at1, Some(Fecha::new(1, 2, 2000)));

        at1.set_prox_fecha(Some(Fecha::new(1, 2, 2000)));

        veterinaria.modificar_diagnostico(&at1, "Nuevo diagnostico".to_string());
        assert_eq!(
            veterinaria.registro_atencion.first().unwrap().diagnostico,
            "Nuevo diagnostico"
        );

        assert_eq!(
            veterinaria.eliminar_atencion(&at1),
            Err(ErrorVeterinaria::EliminarAtencion)
        );
        at1.set_diagnostico("Nuevo diagnostico".to_string());
        assert_eq!(veterinaria.eliminar_atencion(&at1), Ok(()));

        // Intento introducir fecha no valida

        at1.set_prox_fecha(Some(Fecha::new(29, 2, 2021)));
    }

    fn abrir_archivo(path: &String) -> Result<Vec<Atencion>, ErrorVeterinaria> {
        if let Ok(mut f) = File::open(path) {
            let mut buf = String::new();
            f.read_to_string(&mut buf);
            let result: Result<Vec<Atencion>, serde_json::Error> = serde_json::from_str(&buf);

            if let Ok(a) = result {
                return Ok(a);
            }
        }
        Err(ErrorVeterinaria::Archivo)
    }

    #[test]
    fn test_veterinaria_archivo() {
        let mut veterinaria = creacion_veterinaria();

        veterinaria.file_path = "test_files/veterinaria2.json".to_string();

        // Si existia archivo previo, corroboro sus datos

        if let Ok(a) = abrir_archivo(&veterinaria.file_path) {
            assert_eq!(a.len(), 1);
            assert_eq!(a.first().unwrap().mascota.nombre, "Juancito");
        };

        let mascota = Mascota::new(
            "Juancito".to_string(),
            5,
            Animales::Gato,
            Duenio::new(
                "Pedro".to_string(),
                "155".to_string(),
                "2217485463".to_string(),
            ),
        );
        let mut atencion = Atencion::new(
            mascota.clone(),
            "Herida moderada".to_string(),
            "Gasa y alcohol".to_string(),
            Some(Fecha::new(5, 6, 2024)),
        );

        // Registro atencion

        assert!(veterinaria.registrar_atencion(atencion.clone()).is_ok());

        // Modifico diagnositco (el cambio debe apreciarse en el archivo)

        assert!(veterinaria
            .modificar_diagnostico(&atencion, "Pequeña herida".to_string())
            .is_ok());

        // Modifico fecha (el cambio debe apreciarse en el archivo)

        atencion.diagnostico = "Pequeña herida".to_string();
        assert!(veterinaria.modificar_fecha_visita(&atencion, None).is_ok());

        // Intento eliminar la atencion (no la encontrará ya que tiene los datos viejos)

        assert_eq!(
            veterinaria.eliminar_atencion(&atencion),
            Err(ErrorVeterinaria::EliminarAtencion)
        );
    }

    #[test]
    fn test_veterinaria_archivo_errores() {
        let mut veterinaria = creacion_veterinaria();

        veterinaria.file_path = "/".to_string();

        assert!(abrir_archivo(&veterinaria.file_path).is_err());

        let mascota = Mascota::new(
            "Juancito".to_string(),
            5,
            Animales::Gato,
            Duenio::new(
                "Pedro".to_string(),
                "155".to_string(),
                "2217485463".to_string(),
            ),
        );
        let atencion = Atencion::new(
            mascota.clone(),
            "Herida moderada".to_string(),
            "Gasa y alcohol".to_string(),
            Some(Fecha::new(5, 6, 2024)),
        );

        // Fuerzo error de eliminar atencion y modificar atencion (no hay atenciones validas)

        match veterinaria.eliminar_atencion(&atencion) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        match veterinaria.modificar_diagnostico(&atencion, "test".to_string()) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        };

        // Fuerzo el error al crear el archivo con un path no valido

        assert!(veterinaria.registrar_atencion(atencion).is_err());
    }
}
