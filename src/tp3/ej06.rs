use std::f64::{MAX, MIN};

struct Examen {
    materia: String,
    nota: f64,
}

impl Examen {
    fn new(materia: String, nota: f64) -> Examen {
        Examen { materia, nota }
    }
}

struct Estudiante {
    nombre: String,
    id: i32,
    calificaciones: Option<Vec<Examen>>,
}

impl Estudiante {
    fn new(nombre: String, id: i32, calificaciones: Option<Vec<Examen>>) -> Estudiante {
        Estudiante {
            nombre,
            id,
            calificaciones,
        }
    }

    fn obtener_promedio(&self) -> Option<f64> {
        if let Some(v) = &self.calificaciones {
            let mut promedio = 0.0;

            v.iter().for_each(|e| promedio += e.nota);

            match v.len() {
                l if l > 0 => Some(promedio / v.len() as f64),
                _ => None,
            }
        } else {
            None
        }
    }

    fn obtener_calificacion_mas_alta(&self) -> Option<f64> {
        if let Some(v) = &self.calificaciones {
            let mut max_nota = MIN;

            v.iter().for_each(|e| max_nota = max_nota.max(e.nota));

            if max_nota != MIN {
                return Some(max_nota);
            }
        }

        None
    }

    fn obtener_calificacion_mas_baja(&self) -> Option<f64> {
        if let Some(v) = &self.calificaciones {
            let mut min_nota = MAX;

            v.iter().for_each(|e| min_nota = min_nota.min(e.nota));

            if min_nota != MAX {
                return Some(min_nota);
            }
        }

        None
    }
}

#[test]
fn test_estudiante1() {
    let e = Estudiante::new("Alum1".to_string(), 4, Some(Vec::new()));

    assert_eq!(e.obtener_promedio(), None);
    assert_eq!(e.obtener_calificacion_mas_alta(), None);
    assert_eq!(e.obtener_calificacion_mas_alta(), None);
}

#[test]
fn test_estudiante2() {
    let e = Estudiante::new("Alum2".to_string(), 6, None);

    assert_eq!(e.obtener_promedio(), None);
    assert_eq!(e.obtener_calificacion_mas_alta(), None);
    assert_eq!(e.obtener_calificacion_mas_alta(), None);
}

#[test]
fn test_estudiante3() {
    let examenes = vec![
        Examen::new("Mat1".to_string(), 7.5),
        Examen::new("Mat2".to_string(), 4.0),
        Examen::new("Mat3".to_string(), 9.25),
        Examen::new("Mat4".to_string(), 10.0),
        Examen::new("Mat5".to_string(), 3.75),
    ];

    let e = Estudiante::new("Alum3".to_string(), 4, Some(examenes));

    assert_eq!(e.obtener_promedio(), Some(6.9));
    assert_eq!(e.obtener_calificacion_mas_alta(), Some(10.0));
    assert_eq!(e.obtener_calificacion_mas_baja(), Some(3.75));
}
