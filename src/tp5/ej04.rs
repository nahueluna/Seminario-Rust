use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt::Display, fs::File, io::prelude::*, path::Path};

use crate::tp3::ej03::Fecha;

#[derive(Debug)]
struct Biblioteca {
    nombre: String,
    direccion: String,
    copias: HashMap<Libro, u32>,
    prestamos: Vec<Prestamo>,
    path_copias: String,
    path_prestamos: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Eq, Hash)]
struct Libro {
    isbn: u32,
    titulo: String,
    autor: String,
    paginas: u32,
    genero: Generos,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
enum Generos {
    Novela,
    Infantil,
    Tecnico,
    Otros,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Prestamo {
    libro: Libro,
    cliente: Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    fue_devuelto: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}

impl Biblioteca {
    fn new(
        nombre: String,
        direccion: String,
        path_copias: String,
        path_prestamos: String,
    ) -> Biblioteca {
        Biblioteca {
            nombre,
            direccion,
            copias: HashMap::new(),
            prestamos: Vec::new(),
            path_copias,
            path_prestamos,
        }
    }

    fn actualizar_archivo<T>(path: &String, elemento: &T) -> Result<(), ErrorBiblioteca>
    where
        T: ElementoBiblioteca + Serialize,
    {
        if let Ok(mut f) = File::create(path) {
            let Ok(elem) = serde_json::to_string_pretty(&elemento) else {
                return Err(ErrorBiblioteca::Archivo);
            };

            let Ok(_) = f.write_all(&elem.as_bytes()) else {
                return Err(ErrorBiblioteca::Archivo);
            };

            return Ok(());
        }

        Err(ErrorBiblioteca::Archivo)
    }

    fn agregar_copia(&mut self, libro: Libro) -> Result<(), ErrorBiblioteca> {
        self.copias.insert(libro, 0);

        // serde_json::to_string falla para maps con keys que no son strings
        Self::actualizar_archivo(
            &self.path_copias,
            &self
                .copias
                .clone()
                .into_iter()
                .collect::<Vec<(Libro, u32)>>(),
        )
    }

    fn obtener_cantidad_copias(&self, libro: &Libro) -> u32 {
        match self.copias.get(&libro) {
            Some(cant) => *cant,
            None => 0,
        }
    }

    fn decrementar_cantidad_copias(&mut self, libro: &Libro) -> Result<(), ErrorBiblioteca> {
        match self.copias.get_mut(&libro) {
            Some(cant) => {
                if *cant > 0 {
                    *cant -= 1;

                    return Self::actualizar_archivo(
                        &self.path_copias,
                        &self
                            .copias
                            .clone()
                            .into_iter()
                            .collect::<Vec<(Libro, u32)>>(),
                    );
                }
            }
            None => (),
        }

        Err(ErrorBiblioteca::ModificarCantidadCopia)
    }

    fn incrementar_cantidad_copias(&mut self, libro: &Libro) -> Result<(), ErrorBiblioteca> {
        match self.copias.get_mut(&libro) {
            Some(cant) => {
                *cant += 1;

                return Self::actualizar_archivo(
                    &self.path_copias,
                    &self
                        .copias
                        .clone()
                        .into_iter()
                        .collect::<Vec<(Libro, u32)>>(),
                );
            }
            None => Err(ErrorBiblioteca::ModificarCantidadCopia),
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
    ) -> Result<(), ErrorBiblioteca> {
        if !fecha_vencimiento.es_fecha_valida() {
            panic!("Fecha no valida");
        }

        if self.contar_prestamos_cliente(&cliente) <= 5 && self.obtener_cantidad_copias(&libro) >= 1
        {
            match self.decrementar_cantidad_copias(&libro) {
                Ok(_) => (),
                Err(e) => {
                    if e == ErrorBiblioteca::ModificarCantidadCopia {
                        panic!("{}", e)
                    }
                }
            }
            self.prestamos
                .push(Prestamo::new(libro, cliente, fecha_vencimiento));

            return Self::actualizar_archivo(&self.path_prestamos, &self.prestamos);
        }

        Err(ErrorBiblioteca::RealizarPrestamo)
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

    fn devolver_libro(
        &mut self,
        libro: &Libro,
        cliente: &Cliente,
        fecha_actual: Fecha,
    ) -> Result<(), ErrorBiblioteca> {
        if !fecha_actual.es_fecha_valida() {
            panic!("Fecha invalida");
        }

        for p in &mut self.prestamos {
            if p.cliente.eq(cliente) && p.libro.isbn == libro.isbn {
                p.fecha_devolucion = Some(fecha_actual);
                p.fue_devuelto = true;

                match self.incrementar_cantidad_copias(libro) {
                    Ok(_) => (),
                    Err(e) => {
                        if e == ErrorBiblioteca::ModificarCantidadCopia {
                            panic!("{}", e);
                        }
                    }
                }

                return Self::actualizar_archivo(&self.path_prestamos, &self.prestamos);
            }
        }

        Err(ErrorBiblioteca::ModificarPrestamo)
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
}

// En una futura implementación podria utilizarse para establecer comportamiento comun
trait ElementoBiblioteca {}

impl ElementoBiblioteca for Prestamo {}
impl ElementoBiblioteca for Libro {}
impl ElementoBiblioteca for Vec<(Libro, u32)> {}
impl ElementoBiblioteca for Vec<Prestamo> {}

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
}

#[derive(Debug, PartialEq)]
enum ErrorBiblioteca {
    Archivo,
    ModificarCantidadCopia,
    RealizarPrestamo,
    ModificarPrestamo,
}

impl Display for ErrorBiblioteca {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorBiblioteca::Archivo => write!(f, "Error al intentar operar con el archivo"),
            ErrorBiblioteca::ModificarCantidadCopia => write!(
                f,
                "Error al intentar modificar la cantidad de copias de un libro"
            ),
            ErrorBiblioteca::RealizarPrestamo => {
                write!(f, "Error al intentar realizar un prestamo")
            }
            ErrorBiblioteca::ModificarPrestamo => {
                write!(f, "Error al intentar actualizar el estado del prestamo")
            }
        }
    }
}

// tarpaulin no puede llegar al 90% de coverage debido a que no cuenta como analizadas
//las sentencias que ocupan multiples lineas

#[cfg(test)]
mod test {
    use std::{fmt::Debug, process::Termination};

    use serde::de::DeserializeOwned;
    use serde_json::Error;

    use super::*;

    fn creacion_contexto() -> (Biblioteca, Vec<Libro>) {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca UNLP".to_string(),
            "Direccion".to_string(),
            Default::default(),
            Default::default(),
        );
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

        // Agrego 3 copias
        biblioteca.agregar_copia(l1.clone()).expect_err("");
        biblioteca.agregar_copia(l2.clone()).expect_err("");
        biblioteca.agregar_copia(l3.clone()).expect_err("");

        // Incremento su stock. l1 = 10, l2 = 5, l3 = 3
        for i in 1..11 {
            biblioteca.incrementar_cantidad_copias(&l1).expect_err("");
            if i % 2 == 0 {
                biblioteca.incrementar_cantidad_copias(&l2).expect_err("");
            }
            if i % 3 == 0 {
                biblioteca.incrementar_cantidad_copias(&l3).expect_err("");
            }
        }

        (biblioteca, vec![l1, l2, l3])
    }

    #[test]
    fn test_biblioteca_vacia() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca UNLP".to_string(),
            "Direccion".to_string(),
            Default::default(),
            Default::default(),
        );
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

        assert_eq!(
            biblioteca.decrementar_cantidad_copias(&l1),
            Err(ErrorBiblioteca::ModificarCantidadCopia)
        );

        assert_eq!(
            biblioteca.incrementar_cantidad_copias(&l1),
            Err(ErrorBiblioteca::ModificarCantidadCopia)
        );
        assert_eq!(biblioteca.contar_prestamos_cliente(&c1), 0);

        assert_eq!(biblioteca.prestamos_a_vencer(10, &fecha_actual).len(), 0);

        assert_eq!(biblioteca.prestamos_vencidos(&fecha_actual).len(), 0);

        assert!(biblioteca.buscar_prestamo(&l1, &c1).is_none());

        assert_eq!(
            biblioteca.devolver_libro(&l1, &c1, fecha_actual),
            Err(ErrorBiblioteca::ModificarPrestamo)
        );
    }

    #[test]
    fn test_biblioteca1() {
        let mut biblioteca = Biblioteca::new(
            "Biblioteca UNLP".to_string(),
            "Direccion".to_string(),
            Default::default(),
            Default::default(),
        );
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
        biblioteca.agregar_copia(l1.clone()).expect_err("");
        biblioteca.agregar_copia(l2.clone()).expect_err("");
        biblioteca.agregar_copia(l3.clone()).expect_err("");

        // Incremento su stock. l1 = 10, l2 = 5, l3 = 3
        for i in 1..11 {
            biblioteca.incrementar_cantidad_copias(&l1).expect_err("");
            if i % 2 == 0 {
                biblioteca.incrementar_cantidad_copias(&l2).expect_err("");
            }
            if i % 3 == 0 {
                biblioteca.incrementar_cantidad_copias(&l3).expect_err("");
            }
        }

        // Chequeo que la cantidad se haya asignado correctamente
        assert_eq!(*biblioteca.copias.get(&l1).unwrap(), 10);

        // Realiza un prestamo y chequea que la cantidad de copias hayan disminuido
        biblioteca
            .realizar_prestamo(l3.clone(), c1.clone(), Fecha::new(8, 5, 2024))
            .expect_err("");
        assert_eq!(*biblioteca.copias.get(&l3).unwrap(), 2);

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

        biblioteca
            .realizar_prestamo(l1.clone(), c1.clone(), Fecha::new(19, 6, 2024))
            .expect_err("");

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
        match biblioteca.devolver_libro(&l1, &c1, fecha_actual) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
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

        assert_eq!(libro1, libro2);

        let cliente = Cliente::new("Name".to_string(), "Phone".to_string(), "Email".to_string());
        let prestamo = Prestamo::new(libro1, cliente, Fecha::new(1, 1, 2020));

        assert_eq!(prestamo.get_cliente().nombre, "Name");
    }

    fn abrir_archivo<'de, T>(path: &String) -> Result<T, ErrorBiblioteca>
    where
        T: ElementoBiblioteca + DeserializeOwned,
    {
        if let Ok(mut f) = File::open(path) {
            let mut buf = String::new();
            f.read_to_string(&mut buf)
                .expect("No se ha podido leer el archivo");

            let result: Result<T, Error> = serde_json::from_str(&buf);

            if let Ok(elem) = result {
                return Ok(elem);
            }
        }

        Err(ErrorBiblioteca::Archivo)
    }

    #[test]
    fn test_archivo_copias() {
        let datos = creacion_contexto();
        let mut biblioteca = datos.0;
        let libros = datos.1;

        let l1 = libros.first().unwrap().clone();
        let l2 = libros.get(1).unwrap().clone();
        let l3 = libros.last().unwrap().clone();

        biblioteca.path_copias = "test_files/copias1.json".to_string();

        // Corroboro archivo previo

        if let Ok(copias) = abrir_archivo::<Vec<(Libro, u32)>>(&biblioteca.path_copias) {
            let suma_copias = copias.iter().map(|l| l.1).sum::<u32>();
            assert_eq!(suma_copias, 15);
        }

        // Agrego 3 libros

        assert!(biblioteca.agregar_copia(l1.clone()).is_ok());
        assert!(biblioteca.agregar_copia(l2.clone()).is_ok());
        assert!(biblioteca.agregar_copia(l3.clone()).is_ok());

        // Incremento su stock. l1 = 10, l2 = 5, l3 = 3

        for i in 1..11 {
            biblioteca
                .incrementar_cantidad_copias(&l1)
                .expect("No se pudo incrementar");
            if i % 2 == 0 {
                biblioteca
                    .incrementar_cantidad_copias(&l2)
                    .expect("No se pudo incrementar");
            }
            if i % 3 == 0 {
                biblioteca
                    .incrementar_cantidad_copias(&l3)
                    .expect("No se pudo incrementar");
            }
        }

        // Abro el archivo generado para corroborar informacion

        if let Ok(copias) = abrir_archivo::<Vec<(Libro, u32)>>(&biblioteca.path_copias) {
            assert_eq!(copias.len(), 3); // Chequeo cantidad de libros
            let suma_copias = copias.iter().map(|l| l.1).sum::<u32>();
            assert_eq!(suma_copias, 18);
        }

        // Decremento cantidad de copias

        for _i in 1..4 {
            assert!(biblioteca.decrementar_cantidad_copias(&l1).is_ok());
        }
    }

    #[test]
    fn test_archivo_prestamos() {
        let datos = creacion_contexto();
        let mut biblioteca = datos.0;
        let libros = datos.1;

        let l1 = libros.first().unwrap().clone();
        let l2 = libros.get(1).unwrap().clone();
        let l3 = libros.last().unwrap().clone();

        let c1 = Cliente::new(
            "Nahuel".to_string(),
            "2218570392".to_string(),
            "example@gmail.com".to_string(),
        );
        let c2 = Cliente::new(
            "Pedro".to_string(),
            "2212604821".to_string(),
            "test@hotmail.com".to_string(),
        );

        biblioteca.path_copias = "test_files/copias2.json".to_string();
        biblioteca.path_prestamos = "test_files/prestamos1.json".to_string();

        // Corroboro archivos previo

        if let Ok(prestamos) = abrir_archivo::<Vec<Prestamo>>(&biblioteca.path_copias) {
            assert!(prestamos.get(1).unwrap().fue_devuelto);
            assert!(prestamos.get(1).unwrap().fecha_devolucion.is_some());
        }

        if let Ok(copias) = abrir_archivo::<Vec<(Libro, u32)>>(&biblioteca.path_copias) {
            let suma_copias = copias.iter().map(|l| l.1).sum::<u32>();
            assert_eq!(suma_copias, 16); // l3 se presto 2 veces sin devolverse al final
        }

        // Agrego 3 libros

        biblioteca.agregar_copia(l1.clone()).expect("");
        biblioteca.agregar_copia(l2.clone()).expect("");
        biblioteca.agregar_copia(l3.clone()).expect("");

        // Incremento su stock. l1 = 10, l2 = 5, l3 = 3

        for i in 1..11 {
            biblioteca
                .incrementar_cantidad_copias(&l1)
                .expect("No se pudo incrementar");
            if i % 2 == 0 {
                biblioteca
                    .incrementar_cantidad_copias(&l2)
                    .expect("No se pudo incrementar");
            }
            if i % 3 == 0 {
                biblioteca
                    .incrementar_cantidad_copias(&l3)
                    .expect("No se pudo incrementar");
            }
        }

        // Se realizan 3 prestamos

        assert!(biblioteca
            .realizar_prestamo(l3.clone(), c1.clone(), Fecha::new(10, 1, 2024))
            .is_ok());

        assert!(biblioteca
            .realizar_prestamo(l2.clone(), c1.clone(), Fecha::new(15, 1, 2024))
            .is_ok());

        assert!(biblioteca
            .realizar_prestamo(l3.clone(), c2.clone(), Fecha::new(25, 1, 2024))
            .is_ok());

        // Corrobora cantidad de prestamos en el archivo

        if let Ok(prestamos) = abrir_archivo::<Vec<Prestamo>>(&biblioteca.path_copias) {
            assert_eq!(prestamos.len(), 3);
            assert_eq!(prestamos.first().unwrap().cliente.nombre, "Nahuel");
        }

        // Devuelve un libro

        assert!(biblioteca
            .devolver_libro(&l2, &c1, Fecha::new(30, 5, 2024))
            .is_ok());
    }

    #[test]
    fn test_archivo_copias_y_prestamos_error() {
        let mut biblioteca = Biblioteca::new(
            "Biblio".to_string(),
            "520".to_string(),
            "/".to_string(),
            "/".to_string(),
        );

        let l = Libro::new(
            111213,
            "El Hobbit".to_string(),
            "Tolkien".to_string(),
            255,
            Generos::Otros,
        );

        let c = Cliente::new(
            "Nahuel".to_string(),
            "2218570392".to_string(),
            "example@gmail.com".to_string(),
        );

        // Fuerzo error de incapacidad para modificar cantidad de copias

        let e = biblioteca.incrementar_cantidad_copias(&l).unwrap_err();
        assert_eq!(e, ErrorBiblioteca::ModificarCantidadCopia);
        println!("{}", e);

        // Fuerzo error de incapacidad de realizar prestamo

        let e = biblioteca
            .realizar_prestamo(l.clone(), c.clone(), Fecha::new(1, 1, 2024))
            .unwrap_err();
        assert_eq!(e, ErrorBiblioteca::RealizarPrestamo);
        println!("{}", e);

        // Fuerzo error de incapacidad de actualizar prestamo

        let e = biblioteca
            .devolver_libro(&l, &c, Fecha::new(31, 5, 2024))
            .unwrap_err();
        assert_eq!(e, ErrorBiblioteca::ModificarPrestamo);
        println!("{}", e);
    }
}
