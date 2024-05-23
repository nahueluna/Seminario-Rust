use chrono::Local;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem::discriminant;
use std::process::Termination;

use crate::tp3::ej03::Fecha;

struct PlanSuscripcion(f64, u8);

const BASIC_PLAN: PlanSuscripcion = PlanSuscripcion(50.0, 3);
const CLASSIC_PLAN: PlanSuscripcion = PlanSuscripcion(80.0, 6);
const SUPER_PLAN: PlanSuscripcion = PlanSuscripcion(100.0, 12);

struct StreamingRust {
    suscripciones: HashMap<String, Suscripcion>,
}

#[derive(Debug, PartialEq, Clone)]
struct Suscripcion {
    activo: bool,
    tipo_suscripcion: TipoSuscripcion,
    costo: f64,
    duracion: u8,
    fecha_inicio: Fecha,
    usuario: Usuario,
}

#[derive(Debug, PartialEq, Clone)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    telefono: String,
    metodo_pago: MetodoPago,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum TipoSuscripcion {
    Basic,
    Classic,
    Super,
}

#[derive(Debug, Eq, Clone)]
enum MetodoPago {
    Efectivo,
    MercadoPago(DetallePago),
    Credito(DetallePago),
    Transferencia(DetallePago),
    Cripto(DetallePago),
}

#[derive(Debug, PartialEq, Eq, Default, Hash, Clone)]
struct DetallePago {
    cuenta: String,
    titular: String,
}

impl Hash for MetodoPago {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        discriminant(self).hash(state);
    }
}

impl PartialEq for MetodoPago {
    // Ignora el valor asociado al enum
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl MetodoPago {
    fn get_tabla_metodos() -> HashMap<Self, i32> {
        HashMap::from([
            (MetodoPago::Efectivo, 0),
            (MetodoPago::MercadoPago(Default::default()), 0),
            (MetodoPago::Credito(Default::default()), 0),
            (MetodoPago::Transferencia(Default::default()), 0),
            (MetodoPago::Cripto(Default::default()), 0),
        ])
    }
}

impl TipoSuscripcion {
    fn get_tabla_tipos() -> HashMap<Self, i32> {
        HashMap::from([
            (TipoSuscripcion::Basic, 0),
            (TipoSuscripcion::Classic, 0),
            (TipoSuscripcion::Super, 0),
        ])
    }
}

impl Suscripcion {
    fn new(
        tipo_suscripcion: TipoSuscripcion,
        fecha_inicio: Fecha,
        usuario: Usuario,
    ) -> Suscripcion {
        let costo_y_duracion = Suscripcion::calcular_costo_y_duracion(&tipo_suscripcion);

        Suscripcion {
            activo: true,
            tipo_suscripcion,
            costo: costo_y_duracion.0,
            duracion: costo_y_duracion.1,
            fecha_inicio,
            usuario,
        }
    }

    fn get_tipo_suscripcion(&self) -> &TipoSuscripcion {
        &self.tipo_suscripcion
    }

    fn get_costo(&self) -> f64 {
        self.costo
    }

    fn get_duracion(&self) -> u8 {
        self.duracion
    }

    fn get_fecha_inicio(&self) -> &Fecha {
        &self.fecha_inicio
    }

    fn get_usuario(&self) -> &Usuario {
        &self.usuario
    }

    fn calcular_costo_y_duracion(tipo_suscripcion: &TipoSuscripcion) -> PlanSuscripcion {
        match tipo_suscripcion {
            TipoSuscripcion::Basic => BASIC_PLAN,
            TipoSuscripcion::Classic => CLASSIC_PLAN,
            TipoSuscripcion::Super => SUPER_PLAN,
        }
    }

    fn actualizar_datos(&mut self, tipo_suscripcion: TipoSuscripcion) {
        let costo_y_duracion = Suscripcion::calcular_costo_y_duracion(&TipoSuscripcion::Classic);
        self.tipo_suscripcion = tipo_suscripcion;
        self.costo = costo_y_duracion.0;
        self.duracion = costo_y_duracion.1;
        self.fecha_inicio = Fecha::from(Local::now());
    }

    fn upgrade_suscripcion(&mut self) -> bool {
        if self.activo {
            match &self.tipo_suscripcion {
                TipoSuscripcion::Basic => {
                    self.actualizar_datos(TipoSuscripcion::Classic);
                    true
                }
                TipoSuscripcion::Classic => {
                    self.actualizar_datos(TipoSuscripcion::Super);
                    true
                }
                TipoSuscripcion::Super => false,
            }
        } else {
            false
        }
    }

    fn downgrade_suscripcion(&mut self) -> bool {
        if self.activo {
            match &self.tipo_suscripcion {
                TipoSuscripcion::Super => {
                    self.actualizar_datos(TipoSuscripcion::Classic);
                }
                TipoSuscripcion::Classic => {
                    self.actualizar_datos(TipoSuscripcion::Basic);
                }
                TipoSuscripcion::Basic => {
                    self.activo = false;
                }
            }

            return true;
        }

        false
    }

    fn cancelar_suscripcion(&mut self) {
        match self.activo {
            true => self.activo = false,
            false => (),
        };
    }
}

impl Usuario {
    fn new(
        nombre: String,
        apellido: String,
        email: String,
        telefono: String,
        metodo_pago: MetodoPago,
    ) -> Usuario {
        Usuario {
            nombre,
            apellido,
            email,
            telefono,
            metodo_pago,
        }
    }

    fn get_nombre(&self) -> &String {
        &self.nombre
    }

    fn get_apellido(&self) -> &String {
        &self.apellido
    }

    fn get_email(&self) -> &String {
        &self.email
    }

    fn get_telefono(&self) -> &String {
        &self.telefono
    }

    fn get_metodo_pago(&self) -> &MetodoPago {
        &self.metodo_pago
    }
}

impl DetallePago {
    fn new(cuenta: String, titular: String) -> DetallePago {
        DetallePago { cuenta, titular }
    }
}

impl StreamingRust {
    fn new() -> StreamingRust {
        StreamingRust {
            suscripciones: HashMap::new(),
        }
    }

    fn get_suscripcion(&self, email: &String) -> Option<&Suscripcion> {
        self.suscripciones.get(email)
    }

    fn alta_usuario(
        &mut self,
        nombre: String,
        apellido: String,
        email: String,
        telefono: String,
        metodo_pago: MetodoPago,
        tipo_suscripcion: TipoSuscripcion,
    ) -> bool {
        let searched_suscription = self.suscripciones.get_mut(&email);
        let user = Usuario::new(nombre, apellido, email, telefono, metodo_pago);

        if let Some(sub) = searched_suscription {
            if !sub.activo {
                sub.usuario = user;
                sub.activo = true;
            } else {
                return false;
            }
        } else {
            let suscription = Suscripcion::new(tipo_suscripcion, Fecha::from(Local::now()), user);
            self.suscripciones
                .insert(suscription.get_usuario().get_email().clone(), suscription);
        }

        true
    }

    fn upgrade_usuario(&mut self, user_email: &String) -> bool {
        let searched_suscription = self.suscripciones.get_mut(user_email);

        if let Some(s) = searched_suscription {
            s.upgrade_suscripcion()
        } else {
            false
        }
    }

    fn downgrade_usuario(&mut self, user_email: &String) -> bool {
        let searched_suscription = self.suscripciones.get_mut(user_email);

        if let Some(s) = searched_suscription {
            s.downgrade_suscripcion()
        } else {
            false
        }
    }

    fn baja_usuario(&mut self, user_email: &String) -> bool {
        let searched_suscription = self.suscripciones.get_mut(user_email);

        if let Some(s) = searched_suscription {
            if s.activo {
                s.cancelar_suscripcion();
                return true;
            }
        }

        false
    }

    fn determinar_maximo_metodo_pago(metodos: &HashMap<MetodoPago, i32>) -> Option<MetodoPago> {
        match metodos.iter().max_by_key(|m| m.1) {
            Some(m) => {
                if m.1 > &0 {
                    return Some(m.0.clone());
                }
            }
            None => (),
        }
        None
    }

    fn determinar_maximo_suscripciones(
        tipos: &HashMap<TipoSuscripcion, i32>,
    ) -> Option<TipoSuscripcion> {
        match tipos.iter().max_by_key(|t| t.1) {
            Some(t) => {
                if t.1 > &0 {
                    return Some(t.0.clone());
                }
            }
            None => (),
        }
        None
    }

    fn metodo_pago_activo_mas_utilizado(&self) -> Option<MetodoPago> {
        let mut metodos = MetodoPago::get_tabla_metodos();

        self.suscripciones.values().for_each(|s| match s.activo {
            true => *metodos.get_mut(&s.get_usuario().get_metodo_pago()).unwrap() += 1,
            false => (),
        });

        StreamingRust::determinar_maximo_metodo_pago(&metodos)
    }

    fn suscripcion_activa_mas_contratada(&self) -> Option<TipoSuscripcion> {
        let mut tipos = TipoSuscripcion::get_tabla_tipos();

        self.suscripciones.values().for_each(|s| match s.activo {
            true => *tipos.get_mut(&s.tipo_suscripcion).unwrap() += 1,
            false => (),
        });

        StreamingRust::determinar_maximo_suscripciones(&tipos)
    }

    fn metodo_pago_general_mas_utilizado(&self) -> Option<MetodoPago> {
        let mut metodos = MetodoPago::get_tabla_metodos();

        self.suscripciones
            .values()
            .for_each(|s| *metodos.get_mut(s.get_usuario().get_metodo_pago()).unwrap() += 1);

        StreamingRust::determinar_maximo_metodo_pago(&metodos)
    }

    fn suscripcion_general_mas_contratada(&self) -> Option<TipoSuscripcion> {
        let mut tipos = TipoSuscripcion::get_tabla_tipos();

        self.suscripciones
            .values()
            .for_each(|s| *tipos.get_mut(&s.tipo_suscripcion).unwrap() += 1);

        StreamingRust::determinar_maximo_suscripciones(&tipos)
    }
}

////////////////////////////////////////// Tests //////////////////////////////////////////

#[test]
fn alta_y_baja_usuario() {
    // Creo suscripcion con usuario (metodo de pago con detalle)

    let detalle = DetallePago::new("462942".to_string(), "Luna".to_string());
    let user = Usuario::new(
        "Nahuel".to_string(),
        "Luna".to_string(),
        "example@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Cripto(detalle),
    );
    let suscripcion = Suscripcion::new(
        TipoSuscripcion::Classic,
        Fecha::from(Local::now()),
        user.clone(),
    );

    // Compruebo cada campo

    assert!(suscripcion.activo);
    assert_eq!(
        suscripcion.get_tipo_suscripcion(),
        &TipoSuscripcion::Classic
    );
    assert_eq!(suscripcion.get_costo(), 80.0);
    assert_eq!(suscripcion.get_duracion(), 6);
    assert_eq!(suscripcion.get_fecha_inicio(), &Fecha::from(Local::now()));

    assert_eq!(suscripcion.get_usuario().get_nombre(), "Nahuel");
    assert_eq!(suscripcion.get_usuario().get_apellido(), "Luna");
    assert_eq!(suscripcion.get_usuario().get_email(), "example@gmail.com");
    assert_eq!(suscripcion.get_usuario().get_telefono(), "2217482148");
    assert_eq!(
        suscripcion.get_usuario().get_metodo_pago(),
        &MetodoPago::Cripto(Default::default())
    );

    // Instancio StreamingRust e intento baja con estructura vacia

    let mut stream_rust = StreamingRust::new();

    assert!(!stream_rust.baja_usuario(&"example@gmail.com".to_string()));

    // Agrego un usuario

    assert!(stream_rust.alta_usuario(
        user.nombre.clone(),
        user.apellido.clone(),
        user.email.clone(),
        user.telefono.clone(),
        user.metodo_pago.clone(),
        TipoSuscripcion::Classic,
    ));

    // Intento agregar nuevamente al mismo usuario --> no es posible

    assert!(!stream_rust.alta_usuario(
        user.nombre.clone(),
        user.apellido.clone(),
        user.email.clone(),
        user.telefono.clone(),
        user.metodo_pago.clone(),
        TipoSuscripcion::Classic,
    ));

    // Intento agregar otro usuario pero con email ya utilizado --> no es posible

    assert!(!stream_rust.alta_usuario(
        "Gaspar".to_string(),
        "Eliondo".to_string(),
        "example@gmail.com".to_string(),
        "2218488843".to_string(),
        MetodoPago::Efectivo,
        TipoSuscripcion::Super,
    ));

    // Baja usuario existente, intento de baja usuario no activo, intento de baja usuario no existente

    assert!(stream_rust.baja_usuario(&"example@gmail.com".to_string()));

    assert!(!stream_rust.baja_usuario(&"example@gmail.com".to_string()));

    assert!(!stream_rust.baja_usuario(&"juancito@gmail.com".to_string()));

    // Alta usuario ya existente pero inactivo

    assert!(stream_rust.alta_usuario(
        user.nombre.clone(),
        user.apellido.clone(),
        user.email.clone(),
        user.telefono.clone(),
        user.metodo_pago.clone(),
        TipoSuscripcion::Classic,
    ));
}

#[test]
fn test_upgrade_y_downgrade_suscripcion() {
    // Prueba con estructura vacia

    let mut stream_rust = StreamingRust::new();

    assert!(!stream_rust.upgrade_usuario(&"nahuel@gmail.com".to_string()));
    assert!(!stream_rust.downgrade_usuario(&"nahuel@gmail.com".to_string()));

    // Creacion estructuras

    let detalle1 = DetallePago::new("462942".to_string(), "Luna".to_string());
    let user1 = Usuario::new(
        "Nahuel".to_string(),
        "Luna".to_string(),
        "nahuel@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Cripto(detalle1),
    );

    let user2 = Usuario::new(
        "Pedro".to_string(),
        "Gonzalez".to_string(),
        "pedro@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Efectivo,
    );

    let detalle3 = DetallePago::new("75302".to_string(), "Perez".to_string());
    let user3 = Usuario::new(
        "German".to_string(),
        "Perez".to_string(),
        "german@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Cripto(detalle3),
    );

    stream_rust.alta_usuario(
        user1.nombre.clone(),
        user1.apellido.clone(),
        user1.email.clone(),
        user1.telefono.clone(),
        user1.metodo_pago.clone(),
        TipoSuscripcion::Basic,
    );
    stream_rust.alta_usuario(
        user2.nombre.clone(),
        user2.apellido.clone(),
        user2.email.clone(),
        user2.telefono.clone(),
        user2.metodo_pago.clone(),
        TipoSuscripcion::Classic,
    );
    stream_rust.alta_usuario(
        user3.nombre.clone(),
        user3.apellido.clone(),
        user3.email.clone(),
        user3.telefono.clone(),
        user3.metodo_pago.clone(),
        TipoSuscripcion::Super,
    );

    // Upgrade usuario Basic, Classic y Super

    assert!(stream_rust.upgrade_usuario(&user1.email));
    assert_eq!(
        stream_rust
            .get_suscripcion(&user1.email)
            .unwrap()
            .get_tipo_suscripcion(),
        &TipoSuscripcion::Classic
    );

    assert!(stream_rust.upgrade_usuario(&user2.email));
    assert_eq!(
        stream_rust
            .get_suscripcion(&user2.email)
            .unwrap()
            .get_tipo_suscripcion(),
        &TipoSuscripcion::Super
    );

    assert!(!stream_rust.upgrade_usuario(&user3.email));
    assert_eq!(
        stream_rust
            .get_suscripcion(&user3.email)
            .unwrap()
            .get_tipo_suscripcion(),
        &TipoSuscripcion::Super
    );

    // Downgrade usuario Classic y Super

    assert!(stream_rust.downgrade_usuario(&user1.email));
    assert_eq!(
        stream_rust
            .get_suscripcion(&user1.email)
            .unwrap()
            .get_tipo_suscripcion(),
        &TipoSuscripcion::Basic
    );

    assert!(stream_rust.downgrade_usuario(&user2.email));
    assert_eq!(
        stream_rust
            .get_suscripcion(&user2.email)
            .unwrap()
            .get_tipo_suscripcion(),
        &TipoSuscripcion::Classic
    );

    // Downgrade usuario Basic

    assert!(stream_rust.downgrade_usuario(&user1.email));
    assert!(!stream_rust.get_suscripcion(&user1.email).unwrap().activo);

    // Intento de Downgrade y Upgrade a usuario no activo

    assert!(!stream_rust.downgrade_usuario(&user1.email));
    assert!(!stream_rust.upgrade_usuario(&user1.email));
    assert!(!stream_rust.get_suscripcion(&user1.email).unwrap().activo);

    // Upgrade y Downgrade usuario no existente

    assert!(!stream_rust.upgrade_usuario(&"pepe@example.com".to_string()));
    assert!(!stream_rust.downgrade_usuario(&"pepe@example.com".to_string()));
}

#[test]
fn test_metodo_pago_mas_utilizado() {
    // Prueba con estructura vacia

    let mut stream_rust = StreamingRust::new();

    assert_eq!(stream_rust.metodo_pago_activo_mas_utilizado(), None);
    assert_eq!(stream_rust.metodo_pago_general_mas_utilizado(), None);

    // Creacion estructuras

    let detalle1 = DetallePago::new("462942".to_string(), "Luna".to_string());
    let user1 = Usuario::new(
        "Nahuel".to_string(),
        "Luna".to_string(),
        "nahuel@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Cripto(detalle1),
    );

    let user2 = Usuario::new(
        "Pedro".to_string(),
        "Gonzalez".to_string(),
        "pedro@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Efectivo,
    );

    let detalle3 = DetallePago::new("75302".to_string(), "Perez".to_string());
    let user3 = Usuario::new(
        "German".to_string(),
        "Perez".to_string(),
        "german@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Cripto(detalle3),
    );

    stream_rust.alta_usuario(
        user1.nombre.clone(),
        user1.apellido.clone(),
        user1.email.clone(),
        user1.telefono.clone(),
        user1.metodo_pago.clone(),
        TipoSuscripcion::Basic,
    );
    stream_rust.alta_usuario(
        user2.nombre.clone(),
        user2.apellido.clone(),
        user2.email.clone(),
        user2.telefono.clone(),
        user2.metodo_pago.clone(),
        TipoSuscripcion::Classic,
    );
    stream_rust.alta_usuario(
        user3.nombre.clone(),
        user3.apellido.clone(),
        user3.email.clone(),
        user3.telefono.clone(),
        user3.metodo_pago.clone(),
        TipoSuscripcion::Super,
    );

    // Prueba con 3 suscripciones para activos

    assert_eq!(
        stream_rust.metodo_pago_activo_mas_utilizado().unwrap(),
        MetodoPago::Cripto(DetallePago::new(
            "example".to_string(),
            "example".to_string()
        ))
    );

    // Alta dos usuarios mas

    assert!(stream_rust.alta_usuario(
        "Juan".to_string(),
        "Camelo".to_string(),
        "juancame@hotmail.com".to_string(),
        "0117470202".to_string(),
        MetodoPago::Efectivo,
        TipoSuscripcion::Basic,
    ));
    assert!(stream_rust.alta_usuario(
        "Romeo".to_string(),
        "Santos".to_string(),
        "romeosan@yahoo.com".to_string(),
        "2217740022".to_string(),
        MetodoPago::Efectivo,
        TipoSuscripcion::Basic,
    ));

    // Prueba con 5 usuarios para activos

    assert_eq!(
        stream_rust.metodo_pago_activo_mas_utilizado().unwrap(),
        MetodoPago::Efectivo
    );

    // Baja 3 usuarios con metodo de pago Efectivo

    assert!(stream_rust.baja_usuario(&"romeosan@yahoo.com".to_string()));
    assert!(stream_rust.baja_usuario(&"juancame@hotmail.com".to_string()));
    assert!(stream_rust.baja_usuario(&"pedro@gmail.com".to_string()));

    // Prueba con 5 usuarios (2 activos)

    assert_eq!(
        stream_rust.metodo_pago_activo_mas_utilizado().unwrap(),
        MetodoPago::Cripto(Default::default())
    );

    assert_eq!(
        stream_rust.metodo_pago_general_mas_utilizado().unwrap(),
        MetodoPago::Efectivo
    );

    // Baja 2 usuarios que quedan

    assert!(stream_rust.baja_usuario(&"nahuel@gmail.com".to_string()));
    assert!(stream_rust.baja_usuario(&"german@gmail.com".to_string()));

    // Prueba 0 usuarios activos

    assert_eq!(stream_rust.suscripciones.len(), 5); // Todos inactivos, pero siguen almacenados
    assert!(
        !stream_rust
            .get_suscripcion(&"nahuel@gmail.com".to_string())
            .unwrap()
            .activo
    );

    assert_eq!(stream_rust.metodo_pago_activo_mas_utilizado(), None);
    assert_eq!(
        stream_rust.metodo_pago_general_mas_utilizado().unwrap(),
        MetodoPago::Efectivo
    );
}

#[test]
fn test_tipo_suscripcion_mas_contratada() {
    // Prueba con estructura vacia

    let mut stream_rust = StreamingRust::new();

    assert_eq!(stream_rust.suscripcion_activa_mas_contratada(), None);
    assert_eq!(stream_rust.suscripcion_general_mas_contratada(), None);

    // Creacion estructuras

    let detalle1 = DetallePago::new("462942".to_string(), "Luna".to_string());
    let user1 = Usuario::new(
        "Nahuel".to_string(),
        "Luna".to_string(),
        "nahuel@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Cripto(detalle1),
    );

    let user2 = Usuario::new(
        "Pedro".to_string(),
        "Gonzalez".to_string(),
        "pedro@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Efectivo,
    );

    let detalle3 = DetallePago::new("75302".to_string(), "Perez".to_string());
    let user3 = Usuario::new(
        "German".to_string(),
        "Perez".to_string(),
        "german@gmail.com".to_string(),
        "2217482148".to_string(),
        MetodoPago::Cripto(detalle3),
    );

    stream_rust.alta_usuario(
        user1.nombre.clone(),
        user1.apellido.clone(),
        user1.email.clone(),
        user1.telefono.clone(),
        user1.metodo_pago.clone(),
        TipoSuscripcion::Classic,
    );
    stream_rust.alta_usuario(
        user2.nombre.clone(),
        user2.apellido.clone(),
        user2.email.clone(),
        user2.telefono.clone(),
        user2.metodo_pago.clone(),
        TipoSuscripcion::Classic,
    );
    stream_rust.alta_usuario(
        user3.nombre.clone(),
        user3.apellido.clone(),
        user3.email.clone(),
        user3.telefono.clone(),
        user3.metodo_pago.clone(),
        TipoSuscripcion::Super,
    );

    // Prueba con 3 suscripciones para activos

    assert_eq!(
        stream_rust.suscripcion_activa_mas_contratada().unwrap(),
        TipoSuscripcion::Classic
    );

    // Alta dos usuarios mas

    assert!(stream_rust.alta_usuario(
        "Juan".to_string(),
        "Camelo".to_string(),
        "juancame@hotmail.com".to_string(),
        "0117470202".to_string(),
        MetodoPago::Efectivo,
        TipoSuscripcion::Basic,
    ));
    assert!(stream_rust.alta_usuario(
        "Romeo".to_string(),
        "Santos".to_string(),
        "romeosan@yahoo.com".to_string(),
        "2217740022".to_string(),
        MetodoPago::Efectivo,
        TipoSuscripcion::Basic,
    ));

    // Prueba con 5 usuarios para activos

    assert_ne!(
        stream_rust.suscripcion_activa_mas_contratada().unwrap(),
        TipoSuscripcion::Super
    ); // Toma el primer mayor (dada la estructura de HashMap, puede ser cualquiera de los mayores)

    // Alta un usuario mas

    assert!(stream_rust.alta_usuario(
        "Gaspar".to_string(),
        "Eliono".to_string(),
        "gaspi@yahoo.com".to_string(),
        "2216720032".to_string(),
        MetodoPago::MercadoPago(Default::default()),
        TipoSuscripcion::Basic,
    ));

    // Baja 3 usuarios con suscripcion Basic

    assert!(stream_rust.baja_usuario(&"romeosan@yahoo.com".to_string()));
    assert!(stream_rust.baja_usuario(&"juancame@hotmail.com".to_string()));
    assert!(stream_rust.baja_usuario(&"gaspi@yahoo.com".to_string()));

    // Prueba con 5 usuarios (2 activos)

    assert_eq!(
        stream_rust.suscripcion_activa_mas_contratada().unwrap(),
        TipoSuscripcion::Classic
    );

    assert_eq!(
        stream_rust.suscripcion_general_mas_contratada().unwrap(),
        TipoSuscripcion::Basic
    );

    // Baja 3 usuarios que quedan

    assert!(stream_rust.baja_usuario(&"nahuel@gmail.com".to_string()));
    assert!(stream_rust.baja_usuario(&"german@gmail.com".to_string()));
    assert!(stream_rust.baja_usuario(&"pedro@gmail.com".to_string()));

    // Prueba 5 usuarios (0 activos)

    assert_eq!(stream_rust.suscripciones.len(), 6); // Todos inactivos, pero siguen almacenados
    assert!(
        !stream_rust
            .get_suscripcion(&"nahuel@gmail.com".to_string())
            .unwrap()
            .activo
    );

    assert_eq!(stream_rust.suscripcion_activa_mas_contratada(), None);
    assert_eq!(
        stream_rust.suscripcion_general_mas_contratada().unwrap(),
        TipoSuscripcion::Basic
    );
}
