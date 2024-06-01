use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::collections::VecDeque;
use std::io;
use std::{fmt::Display, fs::File, fs::OpenOptions, io::prelude::*, path::Path};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Generos,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
enum Generos {
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

impl Generos {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

struct Playlist {
    nombre: String,
    canciones: VecDeque<Cancion>,
    path_file: String,
}

impl Cancion {
    fn new(titulo: String, artista: String, genero: Generos) -> Cancion {
        Cancion {
            titulo,
            artista,
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

impl Playlist {
    fn new(nombre: String, canciones: VecDeque<Cancion>, path_file: String) -> Playlist {
        Playlist {
            nombre,
            canciones,
            path_file,
        }
    }

    fn build_archivo(&self, canciones: &VecDeque<Cancion>) -> Result<(), ErrorPlaylist> {
        let file = File::create(&self.path_file);

        match file {
            Ok(mut f) => {
                let Ok(c) = serde_json::to_string(&canciones) else {
                    return Err(ErrorPlaylist::FormatearCanciones);
                };

                let Ok(_) = f.write_all(&c.as_bytes()) else {
                    return Err(ErrorPlaylist::EscribirArchivo);
                };

                Ok(())
            }
            Err(_) => return Err(ErrorPlaylist::AbrirArchivo),
        }
    }

    fn agregar_cancion(&mut self, cancion: Cancion) -> Result<(), ErrorPlaylist> {
        self.canciones.push_front(cancion);

        self.build_archivo(&self.canciones)
    }

    fn buscar_cancion(&self, cancion: &Cancion) -> Option<usize> {
        match self.canciones.iter().position(|song| song.eq(cancion)) {
            Some(index) => return Some(index),
            None => (),
        }

        None
    }

    fn eliminar_cancion(&mut self, cancion: &Cancion) -> Result<(), ErrorPlaylist> {
        if let Some(index) = self.buscar_cancion(cancion) {
            self.canciones.remove(index);

            return self.build_archivo(&self.canciones);
        }

        Ok(())
    }

    fn mover_cancion(&mut self, cancion: &Cancion, new_index: usize) -> Result<(), ErrorPlaylist> {
        if let Some(index) = self.buscar_cancion(cancion) {
            let song = self.canciones.remove(index);

            if new_index >= self.canciones.len() {
                self.canciones.push_back(song.unwrap())
            } else {
                self.canciones.insert(new_index, song.unwrap());
            }

            return self.build_archivo(&self.canciones);
        }

        Ok(())
    }

    fn buscar_cancion_por_nombre(&self, titulo_cancion: &String) -> Option<&Cancion> {
        match self
            .canciones
            .iter()
            .position(|song| song.titulo.eq(titulo_cancion))
        {
            Some(index) => return self.canciones.get(index),
            None => (),
        }

        None
    }

    fn get_canciones_por_genero(&self, genero: &Generos) -> Vec<&Cancion> {
        let mut songs = Vec::new();

        self.canciones.iter().for_each(|s| {
            if s.genero.eq(genero) {
                songs.push(s);
            }
        });

        songs
    }

    fn get_canciones_por_artista(&self, artista: &String) -> Vec<&Cancion> {
        let mut songs = Vec::new();

        self.canciones.iter().for_each(|s| {
            if s.artista.eq(artista) {
                songs.push(s);
            }
        });

        songs
    }

    fn modificar_titulo(&mut self, new_title: String) {
        self.nombre = new_title;
    }

    fn clear(&mut self) {
        self.canciones.clear();
    }
}

#[derive(Debug)]
enum ErrorPlaylist {
    AbrirArchivo,
    FormatearCanciones,
    EscribirArchivo,
}

impl Display for ErrorPlaylist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPlaylist::AbrirArchivo => write!(f, "Error al intentar abrir el archivo"),
            ErrorPlaylist::FormatearCanciones => {
                write!(f, "Error al intentar cambiar formato de cancion a string")
            }
            ErrorPlaylist::EscribirArchivo => write!(f, "Error al intentar escribir el archivo"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_playlist_vacia() {
        let c = Cancion::new("Titulo".to_string(), "Artista".to_string(), Generos::Jazz);

        let mut p = Playlist::new("Playlist1".to_string(), VecDeque::new(), Default::default());

        // Operaciones con playlist vacia
        assert_eq!(p.buscar_cancion(&c), None);
        assert!(p.buscar_cancion_por_nombre(&c.titulo).is_none());
        assert!(p.eliminar_cancion(&c).is_ok());
        assert_eq!(p.get_canciones_por_artista(&c.artista).len(), 0);
        assert_eq!(p.get_canciones_por_genero(&c.genero).len(), 0);
        assert!(p.mover_cancion(&c, 15).is_ok()); // No intenta construir el archivo porque la cancion no se encuentra

        // Comprueba que c no haya perdido ownership
        assert_eq!(c.titulo, "Titulo");
        assert_eq!(c.artista, "Artista");
        assert!(c.genero.eq(&Generos::Jazz));
    }

    #[test]
    fn test_playlist1() {
        let v = vec![
            Cancion::new("Cancion1".to_string(), "Artista1".to_string(), Generos::Pop),
            Cancion::new("Cancion2".to_string(), "Artista2".to_string(), Generos::Rap),
            Cancion::new(
                "Cancion3".to_string(),
                "Artista3".to_string(),
                Generos::Rock,
            ),
            Cancion::new("Cancion4".to_string(), "Artista1".to_string(), Generos::Pop),
            Cancion::new("Cancion5".to_string(), "Artista2".to_string(), Generos::Rap),
        ];

        let c2 = Cancion::new("Cancion2".to_string(), "Artista2".to_string(), Generos::Rap);
        let c4 = Cancion::new("Cancion4".to_string(), "Artista1".to_string(), Generos::Pop);

        let mut p = Playlist::new("Playlist".to_string(), VecDeque::new(), Default::default());

        for s in v {
            p.agregar_cancion(s);
        }

        assert_eq!(p.canciones.len(), 5); // Cantidad de canciones
        assert_eq!(p.buscar_cancion(&c2), Some(3)); // Posicion de la cancion
        assert!(p
            .buscar_cancion_por_nombre(&c4.titulo)
            .is_some_and(|s| s.eq(&c4))); // Devuelve la cancion buscada por su titulo
        assert_eq!(p.get_canciones_por_artista(&c2.artista).len(), 2); // Vec con canciones del mismo artista
        assert_eq!(p.get_canciones_por_genero(&c4.genero).len(), 2); // Vec con canciones del mismo género

        p.modificar_titulo("Modified title".to_string());
        assert_eq!(p.nombre, "Modified title");

        assert!(p.mover_cancion(&c2, 0).is_err()); // Path de archivo invalido
        assert!(p.canciones.get(0).is_some_and(|s| s.eq(&c2)));

        assert!(p.canciones.get(3).unwrap().genero.eq(&Generos::Rock)); // Verificar que VecDeque siga teniendo ownership de sus canciones

        assert_eq!(p.canciones.get(4).unwrap().titulo, "Cancion1");

        p.eliminar_cancion(&c4);
        assert_eq!(p.canciones.len(), 4);
        assert_eq!(p.canciones.get(3).unwrap().titulo, "Cancion1");

        p.clear();
        assert_eq!(p.canciones.len(), 0);
    }

    #[test]
    fn test_playlist2() {
        let v = vec![Cancion::new(
            "Cancion".to_string(),
            "Artista".to_string(),
            Generos::Otros,
        )];
        let v_d = VecDeque::from(v);

        let mut p = Playlist::new("Playlist".to_string(), v_d, Default::default());
        let c1 = Cancion::new("Cancion".to_string(), "Artista".to_string(), Generos::Otros);
        let c2 = Cancion::new(
            "Cancion2".to_string(),
            "Artista".to_string(),
            Generos::Otros,
        );

        // No hay un path valido del archivo, pero la operacion pudo realizarse en la estructura
        assert!(p.mover_cancion(&c1, 0).is_err());

        p.agregar_cancion(c2);

        let vec_deq = p.get_canciones_por_artista(&"Artista".to_string());

        assert_eq!(vec_deq.get(0).unwrap().titulo, "Cancion2");
        assert_eq!(vec_deq.get(1).unwrap().titulo, "Cancion");

        assert!(p.mover_cancion(&c1, 28).is_err());
    }

    fn abrir_archivo(path: &String) -> Result<VecDeque<Cancion>, ErrorPlaylist> {
        if let Ok(mut f) = File::open(path) {
            let mut buf = String::new();
            f.read_to_string(&mut buf);

            let result: Result<VecDeque<Cancion>, Error> = serde_json::from_str(&buf);
            if let Ok(c) = result {
                return Ok(c);
            }
        }

        Err(ErrorPlaylist::AbrirArchivo)
    }

    #[test]
    fn test_playlist_archivo() {
        let mut playlist = Playlist::new(
            "Musica".to_string(),
            VecDeque::new(),
            "test_files/canciones.json".to_string(),
        );

        let result = abrir_archivo(&playlist.path_file);

        if let Ok(c) = result {
            assert_eq!(c.len(), 2);
            assert_eq!(c.front().unwrap().titulo, "Cancion3");
        }

        let c1 = Cancion::new("Cancion1".to_string(), "Artista1".to_string(), Generos::Pop);
        let c2 = Cancion::new("Cancion2".to_string(), "Artista2".to_string(), Generos::Rap);
        let c3 = Cancion::new(
            "Cancion3".to_string(),
            "Artista3".to_string(),
            Generos::Rock,
        );

        assert!(playlist.agregar_cancion(c1.clone()).is_ok());
        assert!(playlist.agregar_cancion(c2.clone()).is_ok());
        assert!(playlist.agregar_cancion(c3.clone()).is_ok());

        assert!(playlist.eliminar_cancion(&c2).is_ok());

        assert!(playlist.mover_cancion(&c1, 20).is_ok());

        if let Ok(c) = abrir_archivo(&playlist.path_file) {
            assert_eq!(c.back().unwrap().genero, Generos::Pop);
        }
    }

    #[test]
    fn test_playlist_archivo_error() {
        let mut playlist = Playlist::new("Musica".to_string(), VecDeque::new(), "/".to_string());

        let c1 = Cancion::new("Cancion1".to_string(), "Artista1".to_string(), Generos::Pop);
        let c2 = Cancion::new("Cancion2".to_string(), "Artista2".to_string(), Generos::Rap);

        let result = playlist.agregar_cancion(c1.clone());
        match result {
            Ok(_) => (),
            Err(ref e) => {
                println!("{}", e);
                assert!(result.is_err());
            }
        };

        assert!(playlist.agregar_cancion(c2.clone()).is_err());

        let result = playlist.eliminar_cancion(&c1);
        match result {
            Ok(_) => (),
            Err(ref e) => {
                println!("{}", e);
                assert!(result.is_err());
            }
        };

        let result = playlist.mover_cancion(&c2, 0);
        match result {
            Ok(_) => (),
            Err(ref e) => {
                println!("{}", e);
                assert!(result.is_err());
            }
        };
    }
}
