use crate::tp3::ej03::Fecha;
use chrono::{self, Local};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::io::{self, Error, ErrorKind};
use std::{collections::HashMap, hash::Hash, mem::discriminant};
use std::{fmt::Display, fs::File, fs::OpenOptions, io::prelude::*, path::Path};

#[derive(Debug)]
struct Sistema {
    usuarios: Vec<Usuario>,
    transacciones: Vec<Transaccion>,
    cotizaciones: HashMap<Criptomoneda, f64>,
    path_transacciones: String,
    path_balances: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    validacion: bool,
    balance_fiat: f64,
    balance_cripto: HashMap<String, f64>,
}

#[derive(Debug, Eq, Default, Serialize, Deserialize)]
struct Criptomoneda {
    nombre: String,
    prefijo: String,
    blockchains: Vec<Blockchain>,
}

#[derive(Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize, Clone)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TransaccionFiat {
    usuario: String,
    fecha: Fecha,
    monto: f64,
    medio: Option<MedioPago>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TransaccionCripto {
    usuario: String,
    fecha: Fecha,
    criptomoneda: String,
    monto: f64,
    cotizacion: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TransaccionRetiroRecepcion {
    usuario: String,
    fecha: Fecha,
    blockchain: String,
    hash: Option<String>,
    criptomoneda: String,
    monto: f64,
    cotizacion: f64,
}

#[derive(Debug, Serialize, Deserialize)]
enum Transaccion {
    IngresoDinero(TransaccionFiat),
    RetiroDinero(TransaccionFiat),
    CompraCripto(TransaccionCripto),
    VentaCripto(TransaccionCripto),
    RetiroCripto(TransaccionRetiroRecepcion),
    RecepcionCripto(TransaccionRetiroRecepcion),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum MedioPago {
    Transferencia,
    MercadoPago,
}

impl Sistema {
    fn new(path_transacciones: String, path_balances: String) -> Sistema {
        let mut transacciones = Vec::new();
        if let Ok(t) = Sistema::recuperar_datos_archivo(&path_transacciones) {
            transacciones = t;
        }

        Sistema {
            usuarios: Vec::new(),
            transacciones,
            cotizaciones: Sistema::build_cotizaciones(),
            path_transacciones,
            path_balances,
        }
    }

    fn recuperar_datos_archivo<T>(path: &str) -> Result<T, ErrorSistema>
    where
        T: ElementoSistema + DeserializeOwned,
    {
        if let Ok(mut f) = File::open(path) {
            let mut buf = String::new();
            f.read_to_string(&mut buf)
                .expect("No fue posible leer el archivo");

            if let Ok(elem) = serde_json::from_str(&buf) {
                return Ok(elem);
            }
        }

        Err(ErrorSistema::AbrirArchivo)
    }

    fn actualizar_archivo<T>(path: &str, elemento: &T) -> Result<(), ErrorSistema>
    where
        T: ElementoSistema + Serialize,
    {
        if let Ok(mut f) = File::create(path) {
            let Ok(elem) = serde_json::to_string_pretty(elemento) else {
                return Err(ErrorSistema::FormatoElemento);
            };

            let Ok(_) = f.write_all(&elem.as_bytes()) else {
                return Err(ErrorSistema::EscribirArchivo);
            };

            return Ok(());
        }

        Err(ErrorSistema::AbrirArchivo)
    }

    fn construccion_y_actualizacion_balances(&self) -> Result<(), ErrorSistema> {
        Self::actualizar_archivo(
            &self.path_balances,
            &self.construir_estructura_balances(&self.usuarios),
        )
    }

    fn construir_estructura_balances(
        &self,
        usuarios: &Vec<Usuario>,
    ) -> HashMap<String, (f64, HashMap<String, f64>)> {
        let mut balances = HashMap::new();
        usuarios.iter().for_each(|u| {
            balances.insert(u.dni.clone(), (u.balance_fiat, u.balance_cripto.clone()));
        });

        balances
    }

    fn deconstruir_estructura_balances(
        balances: HashMap<String, (f64, HashMap<String, f64>)>,
        usuarios: &mut Vec<Usuario>,
    ) {
        usuarios.iter_mut().for_each(|u| {
            if let Some(b) = balances.get(&u.dni) {
                u.balance_fiat = b.0;
                u.balance_cripto = b.1.to_owned();
            }
        });
    }

    fn recuperar_balances_usuarios_de_archivo(&mut self) {
        if let Ok(balances) = Sistema::recuperar_datos_archivo(&self.path_balances) {
            Sistema::deconstruir_estructura_balances(balances, &mut self.usuarios);
        }
    }

    fn get_listado_criptos() -> HashMap<String, f64> {
        HashMap::from([
            ("Bitcoin".to_string(), 69_960.95),
            ("Ethereum".to_string(), 3_926.5),
            ("BNB".to_string(), 609.40),
            ("USDT".to_string(), 1.0),
        ])
    }

    fn build_cotizaciones() -> HashMap<Criptomoneda, f64> {
        let lista_criptos = Sistema::get_listado_criptos();
        let mut criptos_cotizaciones = HashMap::new();

        lista_criptos.iter().for_each(|c| {
            criptos_cotizaciones.insert(Criptomoneda::new(c.0.clone()), *c.1);
        });

        criptos_cotizaciones
    }

    fn get_cotizaciones(&self) -> &HashMap<Criptomoneda, f64> {
        &self.cotizaciones
    }

    fn get_usuarios(&self) -> &Vec<Usuario> {
        &self.usuarios
    }

    fn get_transacciones(&self) -> &Vec<Transaccion> {
        &self.transacciones
    }

    fn existe_usuario(&self, dni: &String) -> bool {
        self.usuarios.iter().find(|u| u.dni.eq(dni)).is_none()
    }

    // Corrobora que el usuario no exista en el sistema
    fn agregar_usuario(&mut self, usuario: Usuario) -> Result<(), ErrorSistema> {
        if self.existe_usuario(&usuario.dni) {
            self.usuarios.push(usuario);

            return self.construccion_y_actualizacion_balances();
        }

        Err(ErrorSistema::AgregarUsuario)
    }

    fn agregar_transaccion(&mut self, transaccion: Transaccion) -> Result<(), ErrorSistema> {
        self.transacciones.push(transaccion);

        Sistema::actualizar_archivo(&self.path_transacciones, &self.transacciones)
    }

    fn buscar_usuario(&mut self, dni_usuario: &String) -> Option<&mut Usuario> {
        self.usuarios.iter_mut().find(|u| u.dni.eq(dni_usuario))
    }

    fn get_cripto(&self, nombre_cripto: &String) -> Option<&Criptomoneda> {
        let cripto_buscada = self
            .cotizaciones
            .iter()
            .find(|c| c.0.nombre.eq(nombre_cripto));

        match cripto_buscada {
            Some(cripto) => Some(cripto.0),
            None => None,
        }
    }

    fn get_cotizacion_cripto(&self, cripto: &Criptomoneda) -> f64 {
        match self.cotizaciones.get(cripto) {
            Some(cotizacion) => *cotizacion,
            None => panic!("La criptomoneda no existe en el sistema"), // Se podria mejorar manejando el error
        }
    }

    fn get_tabla_cantidad_criptos() -> HashMap<String, u32> {
        let mut tabla = HashMap::new();
        let criptos = Sistema::get_listado_criptos();

        criptos.iter().for_each(|c| {
            tabla.insert(c.0.clone(), 0);
        });

        tabla
    }

    fn validar_usuario(&mut self, dni_usuario: &String) -> bool {
        let user = self.buscar_usuario(dni_usuario);

        match user {
            Some(u) => {
                u.validar_identidad();
                return true;
            }
            None => (),
        };
        false
    }

    fn actualizar_transaccion_y_balance(
        &mut self,
        transaccion: Transaccion,
    ) -> Result<(), ErrorSistema> {
        let result_transaccion = self.agregar_transaccion(transaccion);
        let result_balance = self.construccion_y_actualizacion_balances();

        match result_transaccion.is_ok() && result_balance.is_ok() {
            true => return Ok(()),
            false => return Err(ErrorSistema::EscribirArchivo),
        }
    }

    fn ingresar_dinero(&mut self, monto: f64, dni_usuario: String) -> Result<(), ErrorSistema> {
        let user = self.buscar_usuario(&dni_usuario);

        match user {
            Some(u) => {
                u.incrementar_balance_fiat(monto);

                let transaccion =
                    Transaccion::IngresoDinero(TransaccionFiat::new(dni_usuario, monto, None));

                return self.actualizar_transaccion_y_balance(transaccion);
            }
            None => (),
        };

        Err(ErrorSistema::ModificacionBalance)
    }

    fn comprar_cripto(
        &mut self,
        monto_fiat: f64,
        cripto: &Criptomoneda,
        dni_usuario: String,
    ) -> Result<(), ErrorSistema> {
        let cotizacion = self.get_cotizacion_cripto(cripto);

        let user = self.buscar_usuario(&dni_usuario);

        match user {
            Some(u) => {
                if u.esta_validado() && u.tiene_balance_suficiente(monto_fiat, None) {
                    u.compra_cripto(monto_fiat, &cripto.nombre, cotizacion);

                    let transaccion = Transaccion::CompraCripto(TransaccionCripto::new(
                        dni_usuario,
                        cripto.nombre.clone(),
                        monto_fiat,
                        cotizacion,
                    ));

                    return self.actualizar_transaccion_y_balance(transaccion);
                }
            }
            None => (),
        }

        Err(ErrorSistema::ModificacionBalance)
    }

    fn vender_cripto(
        &mut self,
        monto_cripto: f64,
        cripto: &Criptomoneda,
        dni_usuario: String,
    ) -> Result<(), ErrorSistema> {
        let cotizacion = self.get_cotizacion_cripto(cripto);

        let user = self.buscar_usuario(&dni_usuario);

        match user {
            Some(u) => {
                if u.esta_validado()
                    && u.tiene_balance_suficiente(monto_cripto, Some(&cripto.nombre))
                {
                    u.compra_fiat(monto_cripto, &cripto.nombre, cotizacion);

                    let transaccion = Transaccion::VentaCripto(TransaccionCripto::new(
                        dni_usuario,
                        cripto.nombre.clone(),
                        monto_cripto,
                        cotizacion,
                    ));

                    return self.actualizar_transaccion_y_balance(transaccion);
                }
            }
            None => (),
        }

        Err(ErrorSistema::ModificacionBalance)
    }

    fn retirar_cripto_a_blockchain(
        &mut self,
        monto: f64,
        cripto: &Criptomoneda,
        dni_usuario: String,
        blockchain: &Blockchain,
    ) -> Result<(), ErrorSistema> {
        if cripto.blockchains.contains(&blockchain) {
            let cotizacion = self.get_cotizacion_cripto(cripto);

            let user = self.buscar_usuario(&dni_usuario);

            match user {
                Some(u) => {
                    if u.esta_validado() && u.tiene_balance_suficiente(monto, Some(&cripto.nombre))
                    {
                        u.decrementar_balance_cripto(monto, &cripto.nombre);

                        let hash = blockchain.nombre.clone() + &rand::random::<u16>().to_string();
                        let transaccion =
                            Transaccion::RetiroCripto(TransaccionRetiroRecepcion::new(
                                dni_usuario,
                                blockchain.nombre.clone(),
                                Some(hash),
                                cripto.nombre.clone(),
                                monto,
                                cotizacion,
                            ));

                        return self.actualizar_transaccion_y_balance(transaccion);
                    }
                }
                None => (),
            }
        }

        Err(ErrorSistema::ModificacionBalance)
    }

    fn recibir_cripto_de_blockchain(
        &mut self,
        monto: f64,
        cripto: &Criptomoneda,
        dni_usuario: String,
        blockchain: &Blockchain,
    ) -> Result<(), ErrorSistema> {
        if cripto.blockchains.contains(&blockchain) {
            let cotizacion = self.get_cotizacion_cripto(cripto);

            let user = self.buscar_usuario(&dni_usuario);

            match user {
                Some(u) => {
                    u.incrementar_balance_cripto(monto, &cripto.nombre);

                    let transaccion =
                        Transaccion::RecepcionCripto(TransaccionRetiroRecepcion::new(
                            dni_usuario,
                            blockchain.nombre.clone(),
                            None,
                            cripto.nombre.clone(),
                            monto,
                            cotizacion,
                        ));

                    return self.actualizar_transaccion_y_balance(transaccion);
                }
                None => (),
            }
        }

        Err(ErrorSistema::ModificacionBalance)
    }

    fn retirar_fiat(
        &mut self,
        monto: f64,
        dni_usuario: String,
        medio_pago: MedioPago,
    ) -> Result<(), ErrorSistema> {
        let user = self.buscar_usuario(&dni_usuario);

        match user {
            Some(u) => {
                if u.esta_validado() && u.tiene_balance_suficiente(monto, None) {
                    u.decrementar_balance_fiat(monto);

                    let transaccion = Transaccion::RetiroDinero(TransaccionFiat::new(
                        dni_usuario,
                        monto,
                        Some(medio_pago),
                    ));

                    return self.actualizar_transaccion_y_balance(transaccion);
                }
            }
            None => (),
        }

        Err(ErrorSistema::ModificacionBalance)
    }

    fn cripto_mas_ventas(&self) -> Option<&Criptomoneda> {
        let mut tabla = Sistema::get_tabla_cantidad_criptos();

        self.transacciones.iter().for_each(|t| match t {
            Transaccion::VentaCripto(info_tr) => {
                *tabla.get_mut(&info_tr.criptomoneda).unwrap() += 1 // La existencia de la criptomoneda es chequeada antes de la creacion de la transaccion
            }
            _ => (),
        });

        let max_cripto = tabla.iter().max_by(|a, b| a.1.cmp(b.1));
        match max_cripto {
            Some(cripto) => {
                if cripto.1 > &0 {
                    self.get_cripto(cripto.0)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn cripto_mas_compras(&self) -> Option<&Criptomoneda> {
        let mut tabla = Sistema::get_tabla_cantidad_criptos();

        self.transacciones.iter().for_each(|t| match t {
            Transaccion::CompraCripto(info_tr) => {
                *tabla.get_mut(&info_tr.criptomoneda).unwrap() += 1
            }
            _ => (),
        });

        let max_cripto = tabla.iter().max_by(|a, b| a.1.cmp(b.1));
        match max_cripto {
            Some(cripto) => {
                if cripto.1 > &0 {
                    self.get_cripto(cripto.0)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn cripto_mayor_volumen_venta(&self) -> Option<&Criptomoneda> {
        let mut tabla: HashMap<String, f64> = self
            .get_cotizaciones()
            .iter()
            .map(|c| (c.0.nombre.clone(), 0.0))
            .collect();

        self.transacciones.iter().for_each(|t| match t {
            Transaccion::VentaCripto(info_tr) => {
                let volumen = info_tr.monto * info_tr.cotizacion; // Monto en cripto * valor cripto en dolares
                *tabla.get_mut(&info_tr.criptomoneda).unwrap() += volumen;
            }
            _ => (),
        });

        let max_cripto = tabla.iter().max_by(|a, b| a.1.total_cmp(b.1));
        match max_cripto {
            Some(cripto) => {
                if cripto.1 > &0.0 {
                    self.get_cripto(cripto.0)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn cripto_mayor_volumen_compra(&self) -> Option<&Criptomoneda> {
        let mut tabla: HashMap<String, f64> = self
            .get_cotizaciones()
            .iter()
            .map(|c| (c.0.nombre.clone(), 0.0))
            .collect();

        self.transacciones.iter().for_each(|t| match t {
            Transaccion::CompraCripto(info_tr) => {
                let volumen = info_tr.monto; // Monto en fiat
                *tabla.get_mut(&info_tr.criptomoneda).unwrap() += volumen;
            }
            _ => (),
        });

        let max_cripto = tabla.iter().max_by(|a, b| a.1.total_cmp(b.1));
        match max_cripto {
            Some(cripto) => {
                if cripto.1 > &0.0 {
                    self.get_cripto(cripto.0)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl Usuario {
    fn new(nombre: String, apellido: String, email: String, dni: String) -> Usuario {
        Usuario {
            nombre,
            apellido,
            email,
            dni,
            validacion: false,
            balance_fiat: 0.0,
            balance_cripto: Usuario::build_balance_cripto(),
        }
    }

    fn get_balance_determinado(&self, cripto: &String) -> Option<&f64> {
        self.balance_cripto.get(cripto)
    }

    fn build_balance_cripto() -> HashMap<String, f64> {
        let mut criptos = Sistema::get_listado_criptos();

        criptos.iter_mut().for_each(|c| *c.1 = 0.0);

        criptos
    }

    fn esta_validado(&self) -> bool {
        self.validacion
    }

    fn tiene_balance_suficiente(&self, monto: f64, cripto: Option<&str>) -> bool {
        if let Some(c) = cripto {
            match self.balance_cripto.get(c) {
                Some(balance) => balance >= &monto,
                None => panic!("Cripto no existente"),
            }
        } else {
            self.balance_fiat >= monto
        }
    }

    fn validar_identidad(&mut self) {
        self.validacion = true;
    }

    fn incrementar_balance_fiat(&mut self, monto_fiat: f64) {
        self.balance_fiat += monto_fiat;
    }

    fn decrementar_balance_fiat(&mut self, monto_fiat: f64) {
        self.balance_fiat -= monto_fiat;
    }

    fn incrementar_balance_cripto(&mut self, monto_cripto: f64, cripto: &String) -> bool {
        match self.balance_cripto.get_mut(cripto) {
            Some(balance) => *balance += monto_cripto,
            None => return false,
        }
        true
    }

    fn decrementar_balance_cripto(&mut self, monto_cripto: f64, cripto: &String) -> bool {
        match self.balance_cripto.get_mut(cripto) {
            Some(balance) => *balance -= monto_cripto,
            None => return false,
        }
        true
    }

    fn compra_fiat(&mut self, monto_cripto: f64, cripto: &String, cotizacion: f64) {
        match self.decrementar_balance_cripto(monto_cripto, cripto) {
            true => self.incrementar_balance_fiat(cotizacion * monto_cripto),
            false => (),
        };
    }

    fn compra_cripto(&mut self, monto_fiat: f64, cripto: &String, cotizacion: f64) {
        match self.incrementar_balance_cripto(monto_fiat / cotizacion, cripto) {
            true => self.decrementar_balance_fiat(monto_fiat),
            false => (),
        };
    }
}

impl PartialEq for Criptomoneda {
    fn eq(&self, other: &Self) -> bool {
        self.nombre.eq(&other.nombre) && self.prefijo.eq(&other.prefijo)
    }
}

impl Hash for Criptomoneda {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.nombre.clone(), self.prefijo.clone()).hash(state);
    }
}

impl Criptomoneda {
    fn new(nombre: String) -> Criptomoneda {
        Criptomoneda {
            nombre: nombre.clone(),
            prefijo: nombre[..3].to_string(),
            blockchains: Vec::new(),
        }
    }

    fn agregar_blockchain(&mut self, blockchain: Blockchain) {
        self.blockchains.push(blockchain)
    }
}

impl Blockchain {
    fn new(nombre: String, prefijo: String) -> Blockchain {
        Blockchain { nombre, prefijo }
    }
}

impl PartialEq for Transaccion {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl TransaccionFiat {
    fn new(dni_usuario: String, monto: f64, medio: Option<MedioPago>) -> TransaccionFiat {
        TransaccionFiat {
            usuario: dni_usuario,
            fecha: Fecha::from(Local::now()),
            monto,
            medio,
        }
    }
}

impl TransaccionCripto {
    fn new(
        dni_usuario: String,
        criptomoneda: String,
        monto: f64,
        cotizacion: f64,
    ) -> TransaccionCripto {
        TransaccionCripto {
            usuario: dni_usuario,
            fecha: Fecha::from(Local::now()),
            criptomoneda,
            monto,
            cotizacion,
        }
    }
}

impl TransaccionRetiroRecepcion {
    fn new(
        dni_usuario: String,
        blockchain: String,
        hash: Option<String>,
        criptomoneda: String,
        monto: f64,
        cotizacion: f64,
    ) -> TransaccionRetiroRecepcion {
        TransaccionRetiroRecepcion {
            usuario: dni_usuario,
            fecha: Fecha::from(Local::now()),
            blockchain,
            hash,
            criptomoneda,
            monto,
            cotizacion,
        }
    }
}

// A futuro puede implementarse comportamiento comun
trait ElementoSistema {}
impl ElementoSistema for Vec<Transaccion> {}
// dni usuario -> (balance fiat, balances cripto)
impl ElementoSistema for HashMap<String, (f64, HashMap<String, f64>)> {}

#[derive(Debug, PartialEq)]
enum ErrorSistema {
    AbrirArchivo,
    EscribirArchivo,
    FormatoElemento,
    AgregarUsuario,
    ModificacionBalance,
}

impl Display for ErrorSistema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorSistema::AbrirArchivo => write!(f, "Error al intentar abrir el archivo"),
            ErrorSistema::EscribirArchivo => write!(f, "Error al intentar escribir el archivo"),
            ErrorSistema::FormatoElemento => {
                write!(f, "Error al intentar formatear a string elemento")
            }
            ErrorSistema::AgregarUsuario => write!(f, "Error al intentar dar de alta un usuario"),
            ErrorSistema::ModificacionBalance => {
                write!(f, "Error al intentar modificar el balance de un usuario")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::format::StrftimeItems;

    use super::*;

    fn creacion_sistema() -> Sistema {
        // Creacion del sistema con 5 usuarios

        let mut sistema = Sistema::new(Default::default(), Default::default());

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );
        let u2 = Usuario::new(
            "Lionel".to_string(),
            "Messi".to_string(),
            "leomessi@gmail.com".to_string(),
            "27427323".to_string(),
        );
        let u3 = Usuario::new(
            "Pedro".to_string(),
            "Gallese".to_string(),
            "pedrito@hotmail.com".to_string(),
            "35587534".to_string(),
        );
        let u4 = Usuario::new(
            "Arda".to_string(),
            "Guler".to_string(),
            "arda@yahoo.com".to_string(),
            "43521534".to_string(),
        );
        let u5 = Usuario::new(
            "Juan".to_string(),
            "Perez".to_string(),
            "juanpe@gmail.com".to_string(),
            "50321572".to_string(),
        );

        let _ = sistema.agregar_usuario(u1);
        let _ = sistema.agregar_usuario(u2);
        let _ = sistema.agregar_usuario(u3);
        let _ = sistema.agregar_usuario(u4);
        let _ = sistema.agregar_usuario(u5);

        sistema
    }

    fn creacion_blockchains() -> Vec<Blockchain> {
        // Creacion de 5 blockchains

        let b1 = Blockchain::new("Binance Smart Chain".to_string(), "BSC".to_string());
        let b2 = Blockchain::new("Cardano".to_string(), "ADA".to_string());
        let b3 = Blockchain::new("Polkadot".to_string(), "DOT".to_string());
        let b4 = Blockchain::new("Solana".to_string(), "SOL".to_string());
        let b5 = Blockchain::new("Ripple".to_string(), "XRP".to_string());
        let b6 = Blockchain::new("Tezos".to_string(), "XTZ".to_string());

        let vec_blockchains = vec![b1, b2, b3, b4, b5, b6];

        vec_blockchains
    }

    fn creacion_criptos(blockchains: &Vec<Blockchain>) -> Vec<Criptomoneda> {
        // Creación de 4 criptomonedas y asociacion con blockchains

        let mut cripto1 = Criptomoneda::new("Bitcoin".to_string());
        cripto1.agregar_blockchain(blockchains.get(0).unwrap().clone());
        cripto1.agregar_blockchain(blockchains.get(1).unwrap().clone());

        let mut cripto2 = Criptomoneda::new("Ethereum".to_string());
        cripto2.agregar_blockchain(blockchains.get(2).unwrap().clone());
        cripto2.agregar_blockchain(blockchains.get(3).unwrap().clone());

        let mut cripto3 = Criptomoneda::new("BNB".to_string());
        cripto3.agregar_blockchain(blockchains.get(4).unwrap().clone());
        cripto3.agregar_blockchain(blockchains.get(5).unwrap().clone());

        let mut cripto4 = Criptomoneda::new("USDT".to_string());
        cripto4.agregar_blockchain(blockchains.get(0).unwrap().clone());
        cripto4.agregar_blockchain(blockchains.get(1).unwrap().clone());
        cripto4.agregar_blockchain(blockchains.get(2).unwrap().clone());
        cripto4.agregar_blockchain(blockchains.get(3).unwrap().clone());
        cripto4.agregar_blockchain(blockchains.get(4).unwrap().clone());
        cripto4.agregar_blockchain(blockchains.get(5).unwrap().clone());

        let criptomonedas = vec![cripto1, cripto2, cripto3, cripto4];

        criptomonedas
    }

    #[test]
    fn test_datos_sistema() {
        let mut sistema = creacion_sistema();

        // Chequeo cantidad de elementos

        assert_eq!(sistema.get_usuarios().len(), 5);
        assert_eq!(sistema.get_transacciones().len(), 0);
        assert_eq!(sistema.get_cotizaciones().len(), 4);

        // Chequeo informacion del usuario es correcta

        assert_eq!(sistema.get_usuarios().first().unwrap().dni, "45497524");
        assert_eq!(sistema.get_usuarios().get(2).unwrap().apellido, "Gallese");
        assert_eq!(
            sistema.get_usuarios().last().unwrap().email,
            "juanpe@gmail.com"
        );

        // Intento agregar un usuario con un DNI ya registrado

        let rep_user = Usuario::new(
            "Pablo".to_string(),
            "Leman".to_string(),
            "test@gmail.com".to_string(),
            "45497524".to_string(),
        );
        assert_eq!(
            sistema.agregar_usuario(rep_user).unwrap_err(),
            ErrorSistema::AgregarUsuario
        );
        assert_eq!(sistema.get_usuarios().len(), 5);

        // Chequeo balances de usuarios sean 0 y tengan las cripto existentes en el sistema

        assert_eq!(sistema.get_usuarios().get(4).unwrap().balance_fiat, 0.0);
        assert!(sistema
            .get_usuarios()
            .get(4)
            .unwrap()
            .balance_cripto
            .contains_key("BNB"));
        assert_eq!(
            sistema
                .get_usuarios()
                .get(1)
                .unwrap()
                .balance_cripto
                .get("Ethereum")
                .unwrap(),
            &0.0
        );

        // Chequeo cotizaciones y criptomonedas del sistema

        assert_eq!(
            sistema.get_cripto(&"Bitcoin".to_string()).unwrap().nombre,
            "Bitcoin"
        );
        assert_eq!(
            sistema.get_cotizacion_cripto(&Criptomoneda::new("USDT".to_string())),
            1.0
        );
    }

    #[test]
    fn test_ingresar_dinero() {
        let mut sistema = creacion_sistema();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );

        // Ingreso dinero y corroboro que se le haya acreditado al usuario

        assert_ne!(
            sistema.ingresar_dinero(499.99, u1.dni.clone()).unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // El error se produce a nivel archivo
        assert_eq!(
            sistema.buscar_usuario(&u1.dni).unwrap().balance_fiat,
            499.99
        );

        // Intento acreditar dinero a usuario no existente

        let u2 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "00000000".to_string(),
        );

        assert_eq!(
            sistema.ingresar_dinero(55.25, u2.dni.clone()).unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        // Chequea transaccion

        assert_eq!(sistema.get_transacciones().len(), 1);

        let transaccion = sistema.get_transacciones().first().unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::IngresoDinero(TransaccionFiat::default())
        );

        if let Transaccion::IngresoDinero(t) = transaccion {
            assert_eq!(t.monto, 499.99);
        }
    }

    #[test]
    fn test_comprar_cripto() {
        let mut sistema = creacion_sistema();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );
        let u5 = Usuario::new(
            "Juan".to_string(),
            "Perez".to_string(),
            "juanpe@gmail.com".to_string(),
            "50321572".to_string(),
        );

        let blockchains = &creacion_blockchains();
        let criptos = creacion_criptos(&blockchains);

        // Valido usuarios e ingreso dinero

        sistema.validar_usuario(&u1.dni);
        sistema.validar_usuario(&u5.dni);

        sistema.ingresar_dinero(1_000.0, u1.dni.clone());
        sistema.ingresar_dinero(120_750.70, u5.dni.clone());

        // Compro cripto para ambos (balances e identidades aptas). Evalúo balances y transacciones

        sistema.comprar_cripto(609.4, criptos.get(2).unwrap(), u1.dni.clone()); // BNB

        sistema.comprar_cripto(34_980.475, criptos.get(0).unwrap(), u5.dni.clone()); // Bitcoin
        sistema.comprar_cripto(39_265.0, criptos.get(1).unwrap(), u5.dni.clone()); // Ethereum

        let u1_sistema = sistema.buscar_usuario(&u1.dni).unwrap();

        assert_eq!(u1_sistema.balance_fiat, 390.6);
        assert_eq!(
            u1_sistema.get_balance_determinado(&"BNB".to_string()),
            Some(&1.0)
        );

        let u5_sistema = sistema.buscar_usuario(&u5.dni).unwrap();

        assert_eq!(u5_sistema.balance_fiat, 46505.225000000006);
        assert_eq!(
            u5_sistema.get_balance_determinado(&"Bitcoin".to_string()),
            Some(&0.5)
        );
        assert_eq!(
            u5_sistema.get_balance_determinado(&"Ethereum".to_string()),
            Some(&10.0)
        );

        let transaccion = sistema.get_transacciones().get(2).unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::CompraCripto(TransaccionCripto::default())
        );

        if let Transaccion::CompraCripto(t) = transaccion {
            assert_eq!(t.cotizacion, 609.4);
        }

        // Compra con usuario no validado

        let u3 = Usuario::new(
            "Pedro".to_string(),
            "Gallese".to_string(),
            "pedrito@hotmail.com".to_string(),
            "35587534".to_string(),
        );

        sistema.ingresar_dinero(10_000.0, u3.dni.clone());
        assert_eq!(
            sistema.buscar_usuario(&u3.dni).unwrap().balance_fiat,
            10_000.0
        );

        assert_eq!(
            sistema
                .comprar_cripto(50.0, criptos.get(3).unwrap(), u3.dni.clone())
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // USDT
        assert_eq!(
            sistema
                .buscar_usuario(&u3.dni)
                .unwrap()
                .get_balance_determinado(&"USDT".to_string())
                .unwrap(),
            &0.0
        );

        // Compra con usuario con saldo insuficiente

        assert_eq!(
            sistema
                .comprar_cripto(500.0, criptos.get(3).unwrap(), u1.dni.clone())
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // USDT
        assert_eq!(
            sistema
                .buscar_usuario(&u1.dni)
                .unwrap()
                .get_balance_determinado(&"USDT".to_string())
                .unwrap(),
            &0.0
        );

        // Compra con usuario no existente

        let u6 = Usuario::new(
            "Gaspar".to_string(),
            "Gonzalez".to_string(),
            "gaspi@hotmail.com".to_string(),
            "64928204".to_string(),
        );

        assert_eq!(
            sistema
                .comprar_cripto(1.0, criptos.get(3).unwrap(), u6.dni.clone())
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );
    }

    #[test]
    fn test_vender_cripto() {
        let mut sistema = creacion_sistema();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );
        let u5 = Usuario::new(
            "Juan".to_string(),
            "Perez".to_string(),
            "juanpe@gmail.com".to_string(),
            "50321572".to_string(),
        );

        let blockchains = creacion_blockchains();
        let criptos = creacion_criptos(&blockchains);

        sistema.validar_usuario(&u1.dni);
        sistema.validar_usuario(&u5.dni);

        sistema.ingresar_dinero(100_000.0, u1.dni.clone());
        sistema.ingresar_dinero(100_000.0, u5.dni.clone());

        sistema.comprar_cripto(69_960.95, criptos.get(0).unwrap(), u1.dni.clone()); // Bitcoin
        sistema.comprar_cripto(30_000.0, criptos.get(3).unwrap(), u1.dni.clone()); // USDT

        sistema.comprar_cripto(39_265.0, criptos.get(1).unwrap(), u5.dni.clone()); // Ethereum
        sistema.comprar_cripto(6094.0, criptos.get(2).unwrap(), u5.dni.clone()); // BNB
        sistema.comprar_cripto(50_000.0, criptos.get(3).unwrap(), u5.dni.clone()); // USDT

        // Vendo cripto de usuarios validados y con balance

        sistema.vender_cripto(1.0, criptos.get(0).unwrap(), u1.dni.clone()); // Bitcoin
        sistema.vender_cripto(25_000.0, criptos.get(3).unwrap(), u1.dni.clone()); // USTD

        sistema.vender_cripto(5.0, criptos.get(1).unwrap(), u5.dni.clone()); // Ethereum

        let u1_sistema = sistema.buscar_usuario(&u1.dni).unwrap();

        assert_eq!(u1_sistema.balance_fiat, 95_000.0);
        assert_eq!(
            u1_sistema.get_balance_determinado(&"USDT".to_string()),
            Some(&5_000.0)
        );

        let u5_sistema = sistema.buscar_usuario(&u5.dni).unwrap();

        assert_eq!(u5_sistema.balance_fiat, 24273.5);
        assert_eq!(
            u5_sistema.get_balance_determinado(&"USDT".to_string()),
            Some(&50_000.0)
        );
        assert_eq!(
            u5_sistema.get_balance_determinado(&"Ethereum".to_string()),
            Some(&5.0)
        );
        // Transaccion de venta 25_000 de u1
        let transaccion = sistema.get_transacciones().get(8).unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::VentaCripto(TransaccionCripto::default())
        );

        if let Transaccion::CompraCripto(t) = transaccion {
            assert_eq!(t.monto, 25_000.0); // Venta 25_000 USDT
        }

        // Compra con usuario no validado

        let u3 = Usuario::new(
            "Pedro".to_string(),
            "Gallese".to_string(),
            "pedrito@hotmail.com".to_string(),
            "35587534".to_string(),
        );

        // Ingreso artificialmente cripto al usuario para comprobar si puede vender
        *sistema
            .buscar_usuario(&u3.dni)
            .unwrap()
            .balance_cripto
            .get_mut(&"USDT".to_string())
            .unwrap() = 10_000.0;

        assert_eq!(
            sistema
                .vender_cripto(10_000.0, criptos.get(3).unwrap(), u3.dni.clone())
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // USDT
        assert_eq!(sistema.buscar_usuario(&u3.dni).unwrap().balance_fiat, 0.0);

        // Compra con usuario con saldo insuficiente

        assert_eq!(
            sistema
                .vender_cripto(10_000.0, criptos.get(3).unwrap(), u1.dni.clone())
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // USDT, le quedan 5_000 USDT
        assert_eq!(
            sistema.buscar_usuario(&u1.dni).unwrap().balance_fiat,
            95_000.0
        ); // Balance fiat no cambio

        // Compra con usuario no existente

        let u6 = Usuario::new(
            "Gaspar".to_string(),
            "Gonzalez".to_string(),
            "gaspi@hotmail.com".to_string(),
            "64928204".to_string(),
        );

        assert_eq!(
            sistema
                .vender_cripto(1.0, criptos.get(3).unwrap(), u6.dni.clone())
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );
    }

    #[test]
    fn test_retirar_cripto() {
        let mut sistema = creacion_sistema();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );
        let u5 = Usuario::new(
            "Juan".to_string(),
            "Perez".to_string(),
            "juanpe@gmail.com".to_string(),
            "50321572".to_string(),
        );

        let blockchains = &creacion_blockchains();
        let criptos = creacion_criptos(&blockchains);

        sistema.validar_usuario(&u1.dni);

        // Introduzco artificalmente balances de criptos

        *sistema
            .buscar_usuario(&u1.dni)
            .unwrap()
            .balance_cripto
            .get_mut(&"Bitcoin".to_string())
            .unwrap() += 5.0; // Posee blockachain 0 y 1

        *sistema
            .buscar_usuario(&u5.dni)
            .unwrap()
            .balance_cripto
            .get_mut(&"Ethereum".to_string())
            .unwrap() += 15.0; // Posee blockachain 2 y 3

        // Retiro cripto de usuario con balance suficiente y validado, con blockchain valido

        assert_ne!(
            sistema
                .retirar_cripto_a_blockchain(
                    3.5,
                    criptos.get(0).unwrap(),
                    u1.dni.clone(),
                    blockchains.get(0).unwrap(),
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // Bitcoin, blockchain 0 | El error proviene de la actualizacion del archivo

        assert_eq!(
            sistema
                .buscar_usuario(&u1.dni)
                .unwrap()
                .get_balance_determinado(&"Bitcoin".to_string())
                .unwrap(),
            &1.5
        ); // Balance del usuario sea el restante esperado

        let transaccion = sistema.get_transacciones().first().unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::RetiroCripto(TransaccionRetiroRecepcion::default())
        );

        if let Transaccion::RetiroCripto(t) = transaccion {
            assert_eq!(t.blockchain, "Binance Smart Chain");
        }

        // Retiro usuario sin validacion

        assert_eq!(
            sistema
                .retirar_cripto_a_blockchain(
                    10.0,
                    criptos.get(1).unwrap(),
                    u5.dni.clone(),
                    blockchains.get(2).unwrap(),
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // Ethereum, blockchain 2

        // Retiro usuario con condiciones pero en blockchain erroneo

        assert_eq!(
            sistema
                .retirar_cripto_a_blockchain(
                    0.5,
                    criptos.get(0).unwrap(),
                    u1.dni.clone(),
                    blockchains.get(3).unwrap(),
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // Bitcoin, blockchain 3 (Bitcoin no opera con él)

        assert_eq!(
            sistema
                .buscar_usuario(&u1.dni)
                .unwrap()
                .get_balance_determinado(&"Bitcoin".to_string())
                .unwrap(),
            &1.5
        ); // Balance del usuario no ha cambiado

        // Retiro usuario sin balance suficiente

        assert_eq!(
            sistema
                .retirar_cripto_a_blockchain(
                    2.0,
                    criptos.get(0).unwrap(),
                    u1.dni.clone(),
                    blockchains.get(1).unwrap(),
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // Bitcoin, blockchain 1. Usuario solo tiene 1.5 Bitcoins

        // Retiro usuario no existente

        let u6 = Usuario::new(
            "Gaspar".to_string(),
            "Gonzalez".to_string(),
            "gaspi@hotmail.com".to_string(),
            "64928204".to_string(),
        );

        assert_eq!(
            sistema
                .retirar_cripto_a_blockchain(
                    1.0,
                    criptos.get(3).unwrap(),
                    u6.dni.clone(),
                    blockchains.get(3).unwrap(),
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // USDT, que opera con todos los blockchains
    }

    #[test]
    fn test_recibir_cripto() {
        let mut sistema = creacion_sistema();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );

        let blockchains = &creacion_blockchains();
        let criptos = creacion_criptos(&blockchains);

        // Ingreso criptos a usuario existente con blockchain correcta

        assert_ne!(
            sistema
                .recibir_cripto_de_blockchain(
                    50.5,
                    criptos.get(3).unwrap(),
                    u1.dni.clone(),
                    blockchains.get(2).unwrap(),
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // Recibe 50.5 USDT, que opera con todas las blockchains | El error se produce por el archivo

        assert_eq!(
            sistema
                .buscar_usuario(&u1.dni)
                .unwrap()
                .get_balance_determinado(&"USDT".to_string())
                .unwrap(),
            &50.5
        ); // Chequeo que se le haya acreditado el balance

        let transaccion = sistema.get_transacciones().first().unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::RecepcionCripto(TransaccionRetiroRecepcion::default())
        ); // Compruebo que la transaccion se haya guardado

        if let Transaccion::RecepcionCripto(t) = transaccion {
            assert_eq!(t.usuario, "45497524");
        } // Compruebo que el dato de la transaccion sea correcto

        // Recibir cripto con blockchain incorrecta

        assert_eq!(
            sistema
                .recibir_cripto_de_blockchain(
                    20.5,
                    criptos.get(2).unwrap(),
                    u1.dni.clone(),
                    blockchains.get(3).unwrap(),
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // BNB, opera con blockchains 4 y 5

        // Recibir cripto con usuario no existente

        let u6 = Usuario::new(
            "Gaspar".to_string(),
            "Gonzalez".to_string(),
            "gaspi@hotmail.com".to_string(),
            "64928204".to_string(),
        );

        assert_eq!(
            sistema
                .recibir_cripto_de_blockchain(
                    5.0,
                    criptos.get(0).unwrap(),
                    u6.dni.clone(),
                    blockchains.get(0).unwrap()
                )
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // Bitcoin, opera con blockchains 0 y 1
    }

    #[test]
    fn test_retirar_fiat() {
        let mut sistema = creacion_sistema();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );
        let u5 = Usuario::new(
            "Juan".to_string(),
            "Perez".to_string(),
            "juanpe@gmail.com".to_string(),
            "50321572".to_string(),
        );

        let blockchains = &creacion_blockchains();
        let criptos = creacion_criptos(&blockchains);

        sistema.validar_usuario(&u1.dni);

        sistema.ingresar_dinero(5000.0, u1.dni.clone());

        sistema.buscar_usuario(&u5.dni).unwrap().balance_fiat += 3500.0; // Introduzco fiat artificialmente a usuario no validado

        // Retiro usuario validado y con balance suficiente

        assert_ne!(
            sistema
                .retirar_fiat(4000.0, u1.dni.clone(), MedioPago::Transferencia)
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        ); // El error se produce por archivo
        assert_eq!(
            sistema.buscar_usuario(&u1.dni).unwrap().balance_fiat,
            1000.0
        );

        let transaccion = sistema.get_transacciones().get(1).unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::RetiroDinero(TransaccionFiat::default())
        );

        if let Transaccion::RetiroDinero(t) = transaccion {
            assert_eq!(t.medio.as_ref().unwrap(), &MedioPago::Transferencia);
        }

        // Retiro usuario sin balance suficiente

        assert_eq!(
            sistema
                .retirar_fiat(1500.0, u1.dni.clone(), MedioPago::MercadoPago)
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        // Retiro usuario sin validar

        assert_eq!(
            sistema
                .retirar_fiat(500.0, u5.dni.clone(), MedioPago::MercadoPago)
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );
        assert_eq!(
            sistema.buscar_usuario(&u5.dni).unwrap().balance_fiat,
            3500.0
        );

        // Retiro usuario inexistente

        let u6 = Usuario::new(
            "Gaspar".to_string(),
            "Gonzalez".to_string(),
            "gaspi@hotmail.com".to_string(),
            "64928204".to_string(),
        );

        assert_eq!(
            sistema
                .retirar_fiat(5.0, u6.dni.clone(), MedioPago::Transferencia)
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );
    }

    #[test]
    fn test_criptos_mayores_estadisticas() {
        let mut sistema = creacion_sistema();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );
        let u5 = Usuario::new(
            "Juan".to_string(),
            "Perez".to_string(),
            "juanpe@gmail.com".to_string(),
            "50321572".to_string(),
        );

        let blockchains = &creacion_blockchains();
        let criptos = creacion_criptos(blockchains);

        sistema.validar_usuario(&u1.dni);
        sistema.validar_usuario(&u5.dni);

        sistema.ingresar_dinero(100_000.0, u1.dni.clone());
        sistema.ingresar_dinero(100_000.0, u5.dni.clone());

        // Usuario 1 - Compra cripto
        sistema.comprar_cripto(50_000.0, criptos.get(0).unwrap(), u1.dni.clone()); // Bitcoin - 0.71
        sistema.comprar_cripto(30_000.0, criptos.get(1).unwrap(), u1.dni.clone()); // Ethereum - 7.64

        // Usuario 2 - Compra cripto
        sistema.comprar_cripto(80_000.0, criptos.get(0).unwrap(), u5.dni.clone()); // Bitcoin - 1.14
        sistema.comprar_cripto(10_000.0, criptos.get(3).unwrap(), u5.dni.clone()); // USDT - 10_000
        sistema.comprar_cripto(5_000.0, criptos.get(1).unwrap(), u5.dni.clone()); // Ethereum - 1.27

        // Usuario 1 - Venta cripto
        sistema.vender_cripto(5.0, criptos.get(1).unwrap(), u1.dni.clone()); // Ethereum

        // Usuario 2 - Venta cripto
        sistema.vender_cripto(1.0, criptos.get(1).unwrap(), u5.dni.clone()); // Ethereum
        sistema.vender_cripto(1.0, criptos.get(0).unwrap(), u5.dni.clone()); // Bitcoin

        // Chequeo de estadisticas
        // Mas ventas
        assert_eq!(
            sistema.cripto_mas_ventas(),
            Some(&Criptomoneda::new("Ethereum".to_string()))
        );

        // Mas compras
        let cripto_mas_comprada = sistema.cripto_mas_compras().unwrap();
        assert!(
            cripto_mas_comprada.eq(&Criptomoneda::new("Ethereum".to_string()))
                || cripto_mas_comprada.eq(&Criptomoneda::new("Bitcoin".to_string()))
        ); // Pueden ser ambas, no es posible saber cual de las dos debido a que la estructura de conteo es un HashMap

        // Mas volumen venta
        assert_eq!(
            sistema.cripto_mayor_volumen_venta(),
            Some(&Criptomoneda::new("Bitcoin".to_string()))
        );

        // Mas volumen compra
        assert_eq!(
            sistema.cripto_mayor_volumen_compra(),
            Some(&Criptomoneda::new("Bitcoin".to_string()))
        );
    }

    #[test]
    fn test_sistema_vacio() {
        let mut sistema = Sistema::new(Default::default(), Default::default());

        let u = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45786392".to_string(),
        );
        let mut c = Criptomoneda::new("USDT".to_string());
        let b = Blockchain::new("Binance Smart Chain".to_string(), "BSC".to_string());
        c.agregar_blockchain(b.clone());

        assert_eq!(
            sistema.ingresar_dinero(10.0, u.dni.clone()).unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        assert_eq!(
            sistema.comprar_cripto(10.0, &c, u.dni.clone()).unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        assert_eq!(
            sistema.vender_cripto(10.0, &c, u.dni.clone()).unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        assert_eq!(
            sistema
                .retirar_cripto_a_blockchain(10.0, &c, u.dni.clone(), &b)
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        assert_eq!(
            sistema
                .recibir_cripto_de_blockchain(10.0, &c, u.dni.clone(), &b)
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        assert_eq!(
            sistema
                .retirar_fiat(10.0, u.dni.clone(), MedioPago::MercadoPago)
                .unwrap_err(),
            ErrorSistema::ModificacionBalance
        );

        assert!(sistema.cripto_mas_ventas().is_none());

        assert!(sistema.cripto_mas_compras().is_none());

        assert!(sistema.cripto_mayor_volumen_venta().is_none());

        assert!(sistema.cripto_mayor_volumen_compra().is_none());
    }

    #[should_panic]
    #[test]
    fn test_get_criptos_inexistentes_y_panics() {
        let plataforma = Sistema::new(Default::default(), Default::default());

        // Get cripto inexistente

        assert!(plataforma.get_cripto(&"test".to_string()).is_none());

        // Get cotizacion cripto inexistente (genera panic!)

        plataforma.get_cotizacion_cripto(&Criptomoneda::new("test".to_string()));

        // Chequeo balance de usuario con cripto inexistente (genera panic!)

        let u = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45876352".to_string(),
        );

        u.tiene_balance_suficiente(10.0, Some("test"));
    }

    #[test]
    fn test_archivo_balances_y_transacciones() {
        // El sistema se crea con path default para evitar que recoja datos del archivo a priori
        let mut sistema = Sistema::new(Default::default(), Default::default());

        sistema.path_balances = "test_files/balances1.json".to_string();
        sistema.path_transacciones = "test_files/transacciones1.json".to_string();

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );
        let u2 = Usuario::new(
            "Lionel".to_string(),
            "Messi".to_string(),
            "leomessi@gmail.com".to_string(),
            "27427323".to_string(),
        );
        let u3 = Usuario::new(
            "Pedro".to_string(),
            "Gallese".to_string(),
            "pedrito@hotmail.com".to_string(),
            "35587534".to_string(),
        );
        let u4 = Usuario::new(
            "Arda".to_string(),
            "Guler".to_string(),
            "arda@yahoo.com".to_string(),
            "43521534".to_string(),
        );
        let u5 = Usuario::new(
            "Juan".to_string(),
            "Perez".to_string(),
            "juanpe@gmail.com".to_string(),
            "50321572".to_string(),
        );

        let b = creacion_blockchains();
        let c = creacion_criptos(&b);

        let bitcoin = c.first().unwrap();
        let usdt = c.get(3).unwrap();

        // Agrego 5 usuarios al sistema

        let _ = sistema.agregar_usuario(u1.clone());
        let _ = sistema.agregar_usuario(u2.clone());
        let _ = sistema.agregar_usuario(u3.clone());
        let _ = sistema.agregar_usuario(u4.clone());
        let _ = sistema.agregar_usuario(u5.clone());

        sistema.validar_usuario(&u1.dni);
        sistema.validar_usuario(&u3.dni);
        sistema.validar_usuario(&u5.dni);

        // Recupero los datos del archivo de balances y corroboro parte de su informacion

        let mut balances: HashMap<String, (f64, HashMap<String, f64>)> =
            Sistema::recuperar_datos_archivo(&sistema.path_balances).unwrap();

        assert_eq!(balances.get(&u1.dni).unwrap().0, 0.0);

        // Modifico balances de usuarios

        // Ingreso dinero

        assert!(sistema.ingresar_dinero(20_000.0, u1.dni.clone()).is_ok());
        assert!(sistema.ingresar_dinero(50_000.0, u3.dni.clone()).is_ok());
        assert!(sistema.ingresar_dinero(35_000.0, u5.dni.clone()).is_ok());

        //Compro cripto

        assert!(sistema
            .comprar_cripto(
                10_000.0,
                &Criptomoneda::new("USDT".to_string()),
                u1.dni.clone()
            )
            .is_ok());
        assert!(sistema
            .comprar_cripto(
                40_000.0,
                &Criptomoneda::new("BNB".to_string()),
                u3.dni.clone()
            )
            .is_ok());
        assert!(sistema
            .comprar_cripto(
                35_000.0,
                &Criptomoneda::new("Bitcoin".to_string()),
                u5.dni.clone()
            )
            .is_ok());

        //Vendo cripto

        assert!(sistema
            .vender_cripto(
                5_000.0,
                &Criptomoneda::new("USDT".to_string()),
                u1.dni.clone()
            )
            .is_ok());
        assert!(sistema
            .vender_cripto(50.0, &Criptomoneda::new("BNB".to_string()), u3.dni.clone())
            .is_ok());

        //Retiro cripto

        assert!(sistema
            .retirar_cripto_a_blockchain(3_000.0, &usdt, u1.dni.clone(), b.first().unwrap(),)
            .is_ok()); // USDT opera en todos los blockchains

        //Recibo cripto

        assert!(sistema
            .recibir_cripto_de_blockchain(0.3, &bitcoin, u3.dni.clone(), b.get(1).unwrap())
            .is_ok()); // Bitcoin opera en blockchains 0 y 1

        //Retiro fiat

        assert!(sistema
            .retirar_fiat(3_000.0, u3.dni.clone(), MedioPago::MercadoPago)
            .is_ok());

        // Chequeo informacion en archivo balances

        balances = Sistema::recuperar_datos_archivo(&sistema.path_balances).unwrap();

        let balances_u1 = balances.get(&u1.dni).unwrap();
        let balances_u3 = balances.get(&u3.dni).unwrap();
        let balances_u5 = balances.get(&u5.dni).unwrap();

        assert_eq!(balances_u1.0, 15_000.0); // Balance fiat
        assert_eq!(balances_u1.1.get(&"USDT".to_string()).unwrap(), &2_000.0); // balance USDT

        assert_eq!(balances_u3.0, 37_470.0); // Balance fiat
        assert!(balances_u3.1.get(&"BNB".to_string()).unwrap() > &15.0); // balance BNB
        assert_eq!(balances_u3.1.get(&"Bitcoin".to_string()).unwrap(), &0.3); // balance Bitcoin

        assert_eq!(balances_u5.0, 0.0); // Balance fiat
        assert!(balances_u5.1.get(&"Bitcoin".to_string()).unwrap() > &0.45); // balance Bitcoin

        // Chequeo informacion en archivo transacciones

        let transacciones: Vec<Transaccion> =
            Sistema::recuperar_datos_archivo(&sistema.path_transacciones).unwrap();

        assert_eq!(transacciones.len(), 11);

        assert_eq!(
            transacciones.first().unwrap(),
            &Transaccion::IngresoDinero(Default::default())
        );
        assert_eq!(
            transacciones.get(6).unwrap(),
            &Transaccion::VentaCripto(Default::default())
        );
        assert_eq!(
            transacciones.last().unwrap(),
            &Transaccion::RetiroDinero(Default::default())
        );
    }

    #[test]
    fn recuperar_archivo_existente() {
        // Creo el sistema y agrego usuario

        let mut sistema = Sistema::new(
            "test_files/transacciones1.json".to_string(),
            Default::default(),
        );

        let u1 = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45497524".to_string(),
        );

        let _ = sistema.agregar_usuario(u1.clone());

        // Lo modifico aca porque no quiero que el alta modifique mi archivo original para consistencia de los test
        sistema.path_balances = "test_files/balances1.json".to_string();

        // Recupero datos del archivo

        sistema.recuperar_balances_usuarios_de_archivo();

        // Chequeo balance

        let u1 = sistema.buscar_usuario(&u1.dni).unwrap(); // El usuario 1 pero del sistema

        // Cargo test ejecuta estos test, tarpaulin no (?)
        assert_eq!(u1.balance_fiat, 15_000.0);
        assert_eq!(
            u1.get_balance_determinado(&"USDT".to_string()),
            Some(&2_000.0)
        );

        // Chequeo transacciones

        assert_eq!(sistema.transacciones.len(), 11);
    }

    #[test]
    fn test_recuperacion_balance() {
        // Creo sistema 1, agrego un usuario e ingreso dinero

        let mut sistema = Sistema::new(Default::default(), "test_files/balances2.json".to_string());

        let u = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "email".to_string(),
            "00".to_string(),
        );

        let _ = sistema.agregar_usuario(u.clone());

        let _ = sistema.ingresar_dinero(10.0, u.dni.clone());

        // Creo sistema 2, agrego al mismo usuario y recupero su balance del sistema 1

        let mut sistema2 = Sistema::new(Default::default(), Default::default());
        let _ = sistema2.agregar_usuario(u.clone());

        sistema2.path_balances = "test_files/balances2.json".to_string(); // Se le agrega aqui para no pisar los datos del anterior archivo

        sistema2.recuperar_balances_usuarios_de_archivo();

        assert_eq!(sistema2.buscar_usuario(&u.dni).unwrap().balance_fiat, 10.0);
    }
}
