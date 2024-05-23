use std::{
    f64::{MAX, MIN},
    u8::MAX as otherMax,
    vec,
};

#[derive(Debug, PartialEq, Clone)]
struct Persona<'a> {
    nombre: &'a str,
    apellido: &'a str,
    direccion: &'a str,
    ciudad: &'a str,
    salario: f64,
    edad: u8,
}

impl<'a> Persona<'a> {
    fn new(
        nombre: &'a str,
        apellido: &'a str,
        direccion: &'a str,
        ciudad: &'a str,
        salario: f64,
        edad: u8,
    ) -> Persona<'a> {
        Persona {
            nombre,
            apellido,
            direccion,
            ciudad,
            salario,
            edad,
        }
    }
}

fn personas_salarios_mayores<'a>(personas: &'a Vec<Persona>, salario: f64) -> Vec<&'a Persona<'a>> {
    personas.iter().filter(|p| p.salario > salario).collect()
}

fn personas_mayor_edad_y_misma_ciudad<'a>(
    personas: &'a Vec<Persona>,
    edad: u8,
    ciudad: &str,
) -> Vec<&'a Persona<'a>> {
    personas
        .iter()
        .filter(|p| p.edad > edad && p.ciudad.eq(ciudad))
        .collect()
}

fn todos_viven_en_la_ciudad(personas: &Vec<Persona>, ciudad: &str) -> bool {
    match personas.len() {
        0 => false,
        _ => personas.iter().all(|p| p.ciudad.eq(ciudad)),
    }
}

fn al_menos_uno_vive_en_la_ciudad(personas: &Vec<Persona>, ciudad: &str) -> bool {
    personas.iter().find(|p| p.ciudad.eq(ciudad)).is_some()
}

fn existe_persona(personas: &[Persona], persona: &Persona) -> bool {
    personas.iter().find(|p| *p == persona).is_some()
}

fn edades_personas<const LEN: usize>(personas: &[Persona; LEN]) -> Option<[u8; LEN]> {
    personas
        .iter()
        .map(|p| p.edad)
        .collect::<Vec<u8>>()
        .try_into()
        .ok()
}

fn persona_mayor_y_menor_salario<'a>(personas: &'a Vec<Persona>) -> Option<[&'a Persona<'a>; 2]> {
    // Se usa partial_cmp porque min_by necesita un Ordering
    /*min_by se queda con el menor de dos valores (Ordering::Less). Si ambos son iguales, se comparan las
    edades de forma invertida (p2 a p1) ya que, si p2.edad > p1.edad, la comparación en el then_with
    devuelve Ordering::Greater. Si min_by recibe Ordering::Greater, interpreta que el primer valor que evaluó
    (p1) es mayor y se queda con el segundo (p2) */
    let minimo = personas.iter().min_by(|p1, p2| {
        p1.salario
            .partial_cmp(&p2.salario)
            .expect("Comparacion de salarios no fue posible")
            .then_with(|| {
                p2.edad
                    .partial_cmp(&p1.edad)
                    .expect("Comparacion de edad no fue posible")
            })
    });

    let maximo = personas.iter().max_by(|p1, p2| {
        p1.salario
            .partial_cmp(&p2.salario)
            .expect("Comparacion de salarios no fue posible")
            .then_with(|| {
                p1.edad
                    .partial_cmp(&p2.edad)
                    .expect("Comparacion de edad no fue posible")
            })
    });

    if minimo.is_some() && maximo.is_some() {
        Some([minimo.unwrap(), maximo.unwrap()])
    } else {
        None
    }
}

#[test]
fn test_personas_mayores_salarios() {
    let p1 = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.5, 20);
    let p2 = Persona::new("German", "Gonzalez", "520", "Berisso", 50.8, 35);
    let p3 = Persona::new("Pedro", "Perez", "155", "Ensenada", 1.49, 43);
    let p4 = Persona::new("Juan", "Garcia", "177", "City Bell", 20.5, 17);

    let mut personas = vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()];

    let vector = personas_salarios_mayores(&personas, 10.0);

    assert_eq!(vector.len(), 3);
    assert_eq!(*vector.get(0).unwrap(), &p1);
    assert_eq!(*vector.get(1).unwrap(), &p2);
    assert_eq!(*vector.get(2).unwrap(), &p4);

    assert_eq!(personas_salarios_mayores(&personas, 100.0).len(), 0);

    personas.clear();

    assert_eq!(personas_salarios_mayores(&personas, 0.0).len(), 0);
}

#[test]
fn test_personas_mayor_edad_misma_ciudad() {
    let p1 = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.5, 20);
    let p2 = Persona::new("German", "Gonzalez", "520", "Berisso", 50.8, 35);
    let p3 = Persona::new("Pedro", "Perez", "155", "La Plata", 1.49, 43);
    let p4 = Persona::new("Juan", "Garcia", "177", "City Bell", 20.5, 17);

    let mut personas = vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()];

    let vector = personas_mayor_edad_y_misma_ciudad(&personas, 25, "La Plata");

    assert_eq!(vector.len(), 1);
    assert_eq!(*vector.first().unwrap(), &p3);

    assert_eq!(
        personas_mayor_edad_y_misma_ciudad(&personas, 1, "Lanus").len(),
        0
    );
    assert_eq!(
        personas_mayor_edad_y_misma_ciudad(&personas, 18, "City Bell").len(),
        0
    );

    personas.clear();

    assert_eq!(
        personas_mayor_edad_y_misma_ciudad(&personas, 0, "La Plata").len(),
        0
    );
}

#[test]
fn test_todos_viven_en_la_ciudad() {
    let p1 = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.5, 20);
    let p2 = Persona::new("German", "Gonzalez", "520", "Berisso", 50.8, 35);
    let p3 = Persona::new("Pedro", "Perez", "155", "La Plata", 1.49, 43);
    let p4 = Persona::new("Juan", "Garcia", "177", "City Bell", 20.5, 17);

    let mut personas = vec![p1, p3];

    assert!(todos_viven_en_la_ciudad(&personas, "La Plata"));

    personas.push(p2);
    personas.push(p4);

    assert!(!todos_viven_en_la_ciudad(&personas, "La Plata"));

    personas.clear();

    assert!(!todos_viven_en_la_ciudad(&personas, "City Bell"));
}

#[test]
fn alguien_vive_en_la_ciudad() {
    let p1 = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.5, 20);
    let p2 = Persona::new("German", "Gonzalez", "520", "Berisso", 50.8, 35);
    let p3 = Persona::new("Pedro", "Perez", "155", "La Plata", 1.49, 43);
    let p4 = Persona::new("Juan", "Garcia", "177", "City Bell", 20.5, 17);

    let mut personas = vec![p1.clone(), p2.clone(), p3.clone(), p4.clone()];

    assert!(al_menos_uno_vive_en_la_ciudad(&personas, "City Bell"));
    assert!(al_menos_uno_vive_en_la_ciudad(&personas, "La Plata"));
    assert!(!al_menos_uno_vive_en_la_ciudad(&personas, "Lanus"));

    personas.clear();

    assert!(!al_menos_uno_vive_en_la_ciudad(&personas, "La Plata"));
}

#[test]
fn test_existe_persona() {
    let p1 = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.5, 20);
    let p2 = Persona::new("German", "Gonzalez", "520", "Berisso", 50.8, 35);
    let p3 = Persona::new("Pedro", "Perez", "155", "La Plata", 1.49, 43);
    let p4 = Persona::new("Juan", "Garcia", "177", "City Bell", 20.5, 17);

    let personas = [p1.clone(), p3.clone(), p4.clone()];

    assert!(existe_persona(&personas, &p1));
    assert!(existe_persona(&personas, &p4));

    assert!(!existe_persona(&personas, &p2));

    let personas2: [Persona; 0] = Default::default();

    assert!(!existe_persona(&personas2, &p1));
}

#[test]
fn test_edades_personas() {
    let p1 = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.5, 20);
    let p2 = Persona::new("German", "Gonzalez", "520", "Berisso", 50.8, 35);
    let p3 = Persona::new("Pedro", "Perez", "155", "La Plata", 1.49, 43);
    let p4 = Persona::new("Juan", "Garcia", "177", "City Bell", 20.5, 17);

    let personas = [p1.clone(), p2.clone(), p3.clone(), p4.clone()];

    assert_eq!(edades_personas(&personas).unwrap(), [20, 35, 43, 17]);

    let personas2: [Persona; 0] = Default::default();
    assert_eq!(edades_personas(&personas2).unwrap(), []);
}

#[test]
fn test_persona_mayor_y_menor_salario() {
    let p1 = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.5, 20);
    let p2 = Persona::new("German", "Gonzalez", "520", "Berisso", 50.8, 35);
    let p3 = Persona::new("Pedro", "Perez", "155", "La Plata", 1.49, 43);
    let p4 = Persona::new("Juan", "Garcia", "177", "City Bell", 50.8, 49);
    let p5 = Persona::new("Ignacio", "Caronte", "Mitre", "CABA", 50.8, 36);
    let p6 = Persona::new("Santiago", "Guten", "Coronel", "Avellaneda", 1.49, 45);

    let mut personas = vec![
        p1.clone(),
        p2.clone(),
        p3.clone(),
        p4.clone(),
        p5.clone(),
        p6.clone(),
    ];

    let resul = persona_mayor_y_menor_salario(&personas);

    assert_eq!(*resul.unwrap()[0], p6);
    assert_eq!(*resul.unwrap()[1], p4);

    // Compruebo no pierdo ownership
    assert_eq!(*personas.get(3).unwrap(), p4);

    personas.clear();

    assert_eq!(persona_mayor_y_menor_salario(&personas), None);
}

#[test]
fn test_lifetime() {
    let vector;

    let mut personas = Vec::new();

    {
        let p = Persona::new("Nahuel", "Luna", "162", "La Plata", 10.0, 20);
        personas.push(p);
        vector = personas_salarios_mayores(&personas, 9.99);

        assert_eq!(personas.first().unwrap(), *vector.first().unwrap());
    }

    assert_eq!(vector.first().unwrap().nombre, "Nahuel");
}
