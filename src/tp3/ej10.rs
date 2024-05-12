use std::{
    collections::{hash_map, HashMap},
    ops::Deref,
    thread::panicking,
};

use super::ej03::Fecha;

#[derive(Debug)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    copias: HashMap<u32, u32>,
    prestamos: Vec<Prestamo>,
}

#[derive(Debug)]
struct Libro {
    isbn: u32,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Generos,
}

#[derive(Debug, Clone)]
enum Generos {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    fue_devuelto: bool,
}

#[derive(Debug)]
struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}

impl Biblioteca {
    fn new(nombre: String, direccion: String) -> Biblioteca {
        Biblioteca {
            nombre,
            direccion,
            copias: HashMap::new(),
            prestamos: Vec::new(),
        }
    }

    fn agregar_copia(&mut self, libro: Libro) {
        self.copias.insert(libro.isbn, 0);
    }

    fn obtener_cantidad_copias(&self, libro: &Libro) -> u32 {
        match self.copias.get(&libro.isbn) {
            Some(cant) => *cant,
            None => 0,
        }
    }

    fn decrementar_cantidad_copias(&mut self, libro: &Libro) -> bool {
        match self.copias.get_mut(&libro.isbn) {
            Some(cant) => {
                if *cant > 0 {
                    *cant -= 1;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn incrementar_cantidad_copias(&mut self, libro: &Libro) -> bool {
        match self.copias.get_mut(&libro.isbn) {
            Some(cant) => {
                *cant += 1;
                true
            }
            None => false,
        }
    }

    fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut cantidad_prestamos = 0;

        for p in &self.prestamos {
            if p.get_cliente().eq(cliente) && !p.fue_devuelto {
                cantidad_prestamos += 1;
            }
        }

        cantidad_prestamos
    }

    fn realizar_prestamo(
        &mut self,
        libro: Libro,
        cliente: Cliente,
        fecha_vencimiento: Fecha,
    ) -> bool {
        if !fecha_vencimiento.es_fecha_valida() {
            panic!("Fecha no valida");
        }

        if self.contar_prestamos_cliente(&cliente) <= 5 && self.obtener_cantidad_copias(&libro) >= 1
        {
            self.decrementar_cantidad_copias(&libro);
            self.prestamos
                .push(Prestamo::new(libro, cliente, fecha_vencimiento));
            true
        } else {
            false
        }
    }

    fn prestamos_a_vencer(&self, dias: u32, fecha_actual: &Fecha) -> Vec<&Prestamo> {
        let mut vec = Vec::new();
        let mut fecha = fecha_actual.clone();
        fecha.sumar_dias(dias + 1);

        for p in &self.prestamos {
            if fecha.es_mayor(&p.fecha_vencimiento) {
                vec.push(p);
            }
        }

        vec
    }

    fn prestamos_vencidos(&self, fecha_actual: &Fecha) -> Vec<&Prestamo> {
        let mut vec = Vec::new();

        for p in &self.prestamos {
            if fecha_actual.es_mayor(&p.fecha_vencimiento) {
                vec.push(p)
            }
        }

        vec
    }

    fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        for p in &self.prestamos {
            if p.libro.isbn == libro.isbn && p.cliente.eq(cliente) {
                return Some(p);
            }
        }

        None
    }

    fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha_actual: Fecha) -> bool {
        if !fecha_actual.es_fecha_valida() {
            panic!("Fecha invalida");
        }

        for p in &mut self.prestamos {
            if p.cliente.eq(cliente) && p.libro.isbn == libro.isbn {
                p.fecha_devolucion = Some(fecha_actual);
                p.fue_devuelto = true;
                self.incrementar_cantidad_copias(libro);
                return true;
            }
        }

        false
    }
}

impl Clone for Libro {
    fn clone(&self) -> Self {
        Libro::new(
            self.isbn,
            self.titulo.clone(),
            self.autor.clone(),
            self.paginas,
            self.genero.clone(),
        )
    }
}

impl Libro {
    fn new(isbn: u32, titulo: String, autor: String, paginas: u32, genero: Generos) -> Libro {
        Libro {
            isbn,
            titulo,
            autor,
            paginas,
            genero,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Prestamo {
    fn new(libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha) -> Prestamo {
        if !fecha_vencimiento.es_fecha_valida() {
            panic!("Fecha no valida")
        }

        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion: None,
            fue_devuelto: false,
        }
    }

    fn get_cliente(&self) -> &Cliente {
        &self.cliente
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Clone for Cliente {
    fn clone(&self) -> Self {
        Cliente::new(
            self.nombre.clone(),
            self.telefono.clone(),
            self.email.clone(),
        )
    }
}

impl Cliente {
    fn new(nombre: String, telefono: String, email: String) -> Cliente {
        Cliente {
            nombre,
            telefono,
            email,
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

#[test]
fn test_biblioteca_vacia() {
    let mut biblioteca = Biblioteca::new("Biblioteca UNLP".to_string(), "Direccion".to_string());
    let l1 = Libro::new(
        123456,
        "1984".to_string(),
        "Orwell".to_string(),
        320,
        Generos::Novela,
    );
    let c1 = Cliente::new(
        "Juan".to_string(),
        "01164829421".to_string(),
        "example@gmail.com".to_string(),
    );
    let fecha_actual = Fecha::new(5, 5, 2024);

    assert_eq!(biblioteca.obtener_cantidad_copias(&l1), 0);
    assert!(!biblioteca.decrementar_cantidad_copias(&l1));
    assert!(!biblioteca.incrementar_cantidad_copias(&l1));
    assert_eq!(biblioteca.contar_prestamos_cliente(&c1), 0);
    assert_eq!(biblioteca.prestamos_a_vencer(10, &fecha_actual).len(), 0);
    assert_eq!(biblioteca.prestamos_vencidos(&fecha_actual).len(), 0);
    assert!(biblioteca.buscar_prestamo(&l1, &c1).is_none());
    assert!(!biblioteca.devolver_libro(&l1, &c1, fecha_actual));
}

#[test]
fn test_biblioteca1() {
    let mut biblioteca = Biblioteca::new("Biblioteca UNLP".to_string(), "Direccion".to_string());
    let l1 = Libro::new(
        123456,
        "1984".to_string(),
        "Orwell".to_string(),
        320,
        Generos::Novela,
    );
    let l2 = Libro::new(
        78910,
        "Battle Royale".to_string(),
        "Koushun Takami".to_string(),
        550,
        Generos::Otros,
    );
    let l3 = Libro::new(
        111213,
        "El Hobbit".to_string(),
        "Tolkien".to_string(),
        255,
        Generos::Otros,
    );

    let c1 = Cliente::new(
        "Juan".to_string(),
        "01164829421".to_string(),
        "example@gmail.com".to_string(),
    );
    let fecha_actual = Fecha::new(5, 5, 2024);

    // Agrego 3 copias
    biblioteca.agregar_copia(l1.clone());
    biblioteca.agregar_copia(l2.clone());
    biblioteca.agregar_copia(l3.clone());

    // Incremento su stock. l1 = 10, l2 = 5, l3 = 3
    for i in 1..11 {
        biblioteca.incrementar_cantidad_copias(&l1);
        if i % 2 == 0 {
            biblioteca.incrementar_cantidad_copias(&l2);
        }
        if i % 3 == 0 {
            biblioteca.incrementar_cantidad_copias(&l3);
        }
    }

    // Chequeo que la cantidad se haya asignado correctamente
    assert_eq!(*biblioteca.copias.get(&l1.isbn).unwrap(), 10);

    // Realiza un prestamo y chequea que la cantidad de copias hayan disminuido
    assert!(biblioteca.realizar_prestamo(l3.clone(), c1.clone(), Fecha::new(8, 5, 2024)));
    assert_eq!(*biblioteca.copias.get(&l3.isbn).unwrap(), 2);

    // Cuento cantidad de prestamos de cliente c1
    assert_eq!(biblioteca.contar_prestamos_cliente(&c1), 1);

    // Chequeo si el prestamo esta proximo a vencer (con un lapso de 3 dias y luego de 1)
    let lista_prestamos_a_vencer = biblioteca.prestamos_a_vencer(3, &fecha_actual);
    assert_eq!(
        lista_prestamos_a_vencer.first().unwrap().libro.titulo,
        "El Hobbit"
    );
    assert_eq!(biblioteca.prestamos_a_vencer(1, &fecha_actual).len(), 0);

    // Verifico prestamos vencidos (primero con la fecha real, luego fuerzo el vencimiento)
    assert_eq!(biblioteca.prestamos_vencidos(&fecha_actual).len(), 0);
    assert_eq!(
        biblioteca
            .prestamos_vencidos(&Fecha::new(9, 5, 2024))
            .first()
            .unwrap()
            .cliente
            .nombre,
        "Juan"
    );

    biblioteca.realizar_prestamo(l1.clone(), c1.clone(), Fecha::new(19, 6, 2024));

    // Busco prestamo. Primero uno existente, despues un libro no prestado, luego un libro prestado pero con cliente incorrecto
    assert_eq!(
        biblioteca.buscar_prestamo(&l1, &c1).unwrap().libro.titulo,
        "1984"
    );
    assert!((biblioteca.buscar_prestamo(&l2, &c1)).is_none());
    assert!(biblioteca
        .buscar_prestamo(
            &l1,
            &Cliente::new(
                "Pedro".to_string(),
                "221843732".to_string(),
                "text@yahoo.com".to_string()
            )
        )
        .is_none());

    // Devuelvo libro y verifico que haya actualizado vector de prestamos
    assert!(biblioteca.devolver_libro(&l1, &c1, fecha_actual));
    assert!(biblioteca.prestamos.last().unwrap().fue_devuelto);
    assert!(biblioteca
        .prestamos
        .last()
        .unwrap()
        .fecha_devolucion
        .as_ref()
        .is_some_and(|f| f.eq(&Fecha::new(5, 5, 2024))));

    // Verifico estado y fecha de libro no devuelto
    assert!(!biblioteca.prestamos.first().unwrap().fue_devuelto);
    assert!(biblioteca
        .prestamos
        .first()
        .unwrap()
        .fecha_devolucion
        .is_none());
}

#[test]
fn test_estructuras_secundarias() {
    let libro1 = Libro {
        isbn: 12345,
        titulo: "example".to_string(),
        autor: "autor".to_string(),
        paginas: 77,
        genero: Generos::Tecnico,
    };

    let libro2 = libro1.clone();

    assert_eq!(libro1.to_string(), libro2.to_string());

    let cliente = Cliente::new("Name".to_string(), "Phone".to_string(), "Email".to_string());
    let prestamo = Prestamo::new(libro1, cliente, Fecha::new(1, 1, 2020));

    assert_eq!(prestamo.get_cliente().nombre, "Name");
}
