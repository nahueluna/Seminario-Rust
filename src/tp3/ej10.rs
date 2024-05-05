use std::{ops::Deref, thread::panicking};

use super::ej03::Fecha;

#[derive(Debug)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    copias: Vec<Copias>,
    prestamos: Vec<Prestamo>,
}

#[derive(Debug)]
struct Copias {
    libro: Libro,
    cantidad: u32,
}

#[derive(Debug)]
struct Libro {
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Generos,
}

#[derive(Debug, PartialEq, Clone)]
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
            copias: Vec::new(),
            prestamos: Vec::new(),
        }
    }

    fn agregar_copia(&mut self, libro: Libro) {
        match self.get_pos_copia(&libro) {
            Some(_) => (),
            None => self.copias.push(Copias::new(libro, 0)),
        }
    }

    fn get_pos_copia(&self, libro: &Libro) -> Option<usize> {
        match self.copias.iter().position(|c| &c.libro == libro) {
            Some(index) => Some(index),
            None => None,
        }
    }

    fn obtener_cantidad_copias(&self, libro: &Libro) -> u32 {
        match self.get_pos_copia(libro) {
            Some(index) => self.copias.get(index).unwrap().get_cantidad(),
            None => 0,
        }
    }

    fn decrementar_cantidad_copias(&mut self, libro: &Libro) -> bool {
        match self.get_pos_copia(libro) {
            Some(index) => match self.copias.get(index).unwrap().get_cantidad() {
                cant if cant > 0 => {
                    self.copias.get_mut(index).unwrap().cantidad -= 1;
                    true
                }
                _ => false,
            },
            None => false,
        }
    }

    fn incrementar_cantidad_copias(&mut self, libro: &Libro) -> bool {
        match self.get_pos_copia(libro) {
            Some(index) => {
                self.copias.get_mut(index).unwrap().cantidad += 1;
                true
            }
            None => false,
        }
    }

    fn contar_prestamos_cliente(&self, cliente: &Cliente) -> u32 {
        let mut cantidad_prestamos = 0;

        for p in &self.prestamos {
            if p.get_cliente() == cliente && !p.fue_devuelto {
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

    fn prestamos_a_vencer(&self, dias: u32, fecha_actual: &Fecha) -> Option<Vec<Prestamo>> {
        let mut vec = Vec::new();
        let mut fecha = fecha_actual.clone();
        fecha.sumar_dias(dias + 1);

        for p in &self.prestamos {
            if fecha.es_mayor(&p.fecha_vencimiento) {
                vec.push(p.clone());
            }
        }

        if vec.len() > 0 {
            Some(vec)
        } else {
            None
        }
    }

    fn prestamos_vencidos(&self, fecha_actual: &Fecha) -> Option<Vec<Prestamo>> {
        let mut vec = Vec::new();

        for p in &self.prestamos {
            if fecha_actual.es_mayor(&p.fecha_vencimiento) {
                vec.push(p.clone())
            }
        }

        if vec.len() > 0 {
            Some(vec)
        } else {
            None
        }
    }

    fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> Option<&Prestamo> {
        for p in &self.prestamos {
            if p.libro.eq(libro) && p.cliente.eq(cliente) {
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
            if p.cliente.eq(cliente) && p.libro.eq(libro) {
                p.fecha_devolucion = Some(fecha_actual);
                p.fue_devuelto = true;
                self.incrementar_cantidad_copias(libro);
                return true;
            }
        }

        false
    }
}

impl Copias {
    fn new(libro: Libro, cantidad: u32) -> Copias {
        Copias { libro, cantidad }
    }

    fn get_cantidad(&self) -> u32 {
        self.cantidad
    }
}

impl PartialEq for Libro {
    fn eq(&self, other: &Self) -> bool {
        self.titulo.eq(&other.titulo)
            && self.autor.eq(&other.autor)
            && self.paginas == other.paginas
            && self.genero.eq(&other.genero)
    }
}

impl Clone for Libro {
    fn clone(&self) -> Self {
        Libro::new(
            self.titulo.clone(),
            self.autor.clone(),
            self.paginas,
            self.genero.clone(),
        )
    }
}

impl Libro {
    fn new(titulo: String, autor: String, paginas: u32, genero: Generos) -> Libro {
        Libro {
            titulo,
            autor,
            paginas,
            genero,
        }
    }
}

impl Clone for Prestamo {
    fn clone(&self) -> Self {
        Prestamo::new(
            self.libro.clone(),
            self.cliente.clone(),
            self.fecha_vencimiento.clone(),
        )
    }
}

impl PartialEq for Prestamo {
    fn eq(&self, other: &Self) -> bool {
        self.libro.eq(&other.libro)
            && self.cliente.eq(&other.cliente)
            && self.fecha_vencimiento.eq(&other.fecha_vencimiento)
    }
}

impl Prestamo {
    fn new(libro: Libro, cliente: Cliente, fecha_vencimiento: Fecha) -> Prestamo {
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

impl PartialEq for Cliente {
    fn eq(&self, other: &Self) -> bool {
        self.nombre.eq(&other.nombre)
            && self.telefono.eq(&other.telefono)
            && self.email.eq(&other.email)
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
}

#[test]
fn test_biblioteca_vacia() {
    let mut biblioteca = Biblioteca::new("Biblioteca UNLP".to_string(), "Direccion".to_string());
    let l1 = Libro::new(
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
    assert_eq!(biblioteca.prestamos_a_vencer(10, &fecha_actual), None);
    assert_eq!(biblioteca.prestamos_vencidos(&fecha_actual), None);
    assert_eq!(biblioteca.buscar_prestamo(&l1, &c1), None);
    assert!(!biblioteca.devolver_libro(&l1, &c1, fecha_actual));
}

#[test]
fn test_biblioteca1() {
    let mut biblioteca = Biblioteca::new("Biblioteca UNLP".to_string(), "Direccion".to_string());
    let l1 = Libro::new(
        "1984".to_string(),
        "Orwell".to_string(),
        320,
        Generos::Novela,
    );
    let l2 = Libro::new(
        "Battle Royale".to_string(),
        "Koushun Takami".to_string(),
        550,
        Generos::Otros,
    );
    let l3 = Libro::new(
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
    assert_eq!(biblioteca.copias.first().unwrap().cantidad, 10);

    // Realiza un prestamo y chequea que la cantidad de copias hayan disminuido
    assert!(biblioteca.realizar_prestamo(l3.clone(), c1.clone(), Fecha::new(8, 5, 2024)));
    assert_eq!(biblioteca.copias.get(2).unwrap().cantidad, 2);

    // Cuento cantidad de prestamos de cliente c1
    assert_eq!(biblioteca.contar_prestamos_cliente(&c1), 1);

    // Chequeo si el prestamo esta proximo a vencer (con un lapso de 3 dias y luego de 1)
    let lista_prestamos_a_vencer = biblioteca.prestamos_a_vencer(3, &fecha_actual).unwrap();
    assert_eq!(
        lista_prestamos_a_vencer.first().unwrap().libro.titulo,
        "El Hobbit"
    );
    assert_eq!(biblioteca.prestamos_a_vencer(1, &fecha_actual), None);

    // Verifico prestamos vencidos (primero con la fecha real, luego fuerzo el vencimiento)
    assert_eq!(biblioteca.prestamos_vencidos(&fecha_actual), None);
    assert_eq!(
        biblioteca
            .prestamos_vencidos(&Fecha::new(9, 5, 2024))
            .unwrap()
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
    assert_eq!((biblioteca.buscar_prestamo(&l2, &c1)), None);
    assert_eq!(
        biblioteca.buscar_prestamo(
            &l1,
            &Cliente::new(
                "Pedro".to_string(),
                "221843732".to_string(),
                "text@yahoo.com".to_string()
            )
        ),
        None
    );

    // Devuelvo libro y verifico que haya actualizado vector de prestamos
    assert!(biblioteca.devolver_libro(&l1, &c1, fecha_actual));
    assert!(biblioteca.prestamos.last().unwrap().fue_devuelto);
    assert_eq!(
        biblioteca.prestamos.last().unwrap().fecha_devolucion,
        Some(Fecha::new(5, 5, 2024))
    );

    // Verifico estado y fecha de libro no devuelto
    assert!(!biblioteca.prestamos.first().unwrap().fue_devuelto);
    assert_eq!(biblioteca.prestamos.first().unwrap().fecha_devolucion, None);
}
