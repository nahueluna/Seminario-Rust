use crate::tp3::ej03::Fecha;
use chrono::{self, Local};
use std::{collections::HashMap, hash::Hash, mem::discriminant};

struct Sistema<'a> {
    usuarios: Vec<Usuario>,
    transacciones: Vec<Transaccion<'a>>,
    cotizaciones: HashMap<Criptomoneda<'a>, f64>,
}

#[derive(Debug, Default)]
struct Usuario {
    nombre: String,
    apellido: String,
    email: String,
    dni: String,
    validacion: bool,
    balance_fiat: f64,
    balance_cripto: HashMap<String, f64>,
}

#[derive(Debug, Eq, Default)]
struct Criptomoneda<'a> {
    nombre: String,
    prefijo: String,
    blockchains: Vec<&'a Blockchain>,
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
struct Blockchain {
    nombre: String,
    prefijo: String,
}

#[derive(Debug)]
struct TransaccionFiat<'a> {
    usuario: &'a Usuario,
    fecha: Fecha,
    monto: f64,
    medio: Option<MedioPago>,
}

#[derive(Debug)]
struct TransaccionCripto<'a> {
    usuario: &'a Usuario,
    fecha: Fecha,
    criptomoneda: &'a Criptomoneda<'a>,
    monto: f64,
    cotizacion: f64,
}

#[derive(Debug)]
struct TransaccionRetiroRecepcion<'a> {
    usuario: &'a Usuario,
    fecha: Fecha,
    blockchain: &'a Blockchain,
    hash: Option<String>,
    criptomoneda: &'a Criptomoneda<'a>,
    monto: f64,
    cotizacion: f64,
}

#[derive(Debug)]
enum Transaccion<'a> {
    IngresoDinero(TransaccionFiat<'a>),
    RetiroDinero(TransaccionFiat<'a>),
    CompraCripto(TransaccionCripto<'a>),
    VentaCripto(TransaccionCripto<'a>),
    RetiroCripto(TransaccionRetiroRecepcion<'a>),
    RecepcionCripto(TransaccionRetiroRecepcion<'a>),
}

#[derive(Debug, PartialEq)]
enum MedioPago {
    Transferencia,
    MercadoPago,
}

impl<'a> Sistema<'a> {
    fn new() -> Sistema<'a> {
        Sistema {
            usuarios: Vec::new(),
            transacciones: Vec::new(),
            cotizaciones: Sistema::build_cotizaciones(),
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

    fn build_cotizaciones() -> HashMap<Criptomoneda<'a>, f64> {
        let lista_criptos = Sistema::get_listado_criptos();
        let mut criptos_cotizaciones = HashMap::new();

        lista_criptos.iter().for_each(|c| {
            criptos_cotizaciones.insert(Criptomoneda::new(c.0.clone()), *c.1);
        });

        criptos_cotizaciones
    }

    fn get_cotizaciones(&self) -> &HashMap<Criptomoneda<'a>, f64> {
        &self.cotizaciones
    }

    fn get_usuarios(&self) -> &Vec<Usuario> {
        &self.usuarios
    }

    fn get_transacciones(&self) -> &Vec<Transaccion> {
        &self.transacciones
    }

    // Corrobora que el usuario no exista en el sistema
    fn agregar_usuario(&mut self, usuario: Usuario) -> bool {
        if self
            .usuarios
            .iter()
            .find(|u| u.dni.eq(&usuario.dni))
            .is_none()
        {
            self.usuarios.push(usuario);
            true
        } else {
            false
        }
    }

    fn agregar_transaccion(&mut self, transaccion: Transaccion<'a>) {
        self.transacciones.push(transaccion);
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
            None => panic!("La criptomoneda no existe en el sistema"),
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

    fn ingresar_dinero(&mut self, monto: f64, usuario: &'a Usuario) -> bool {
        let user = self.buscar_usuario(&usuario.dni);

        match user {
            Some(u) => {
                u.incrementar_balance_fiat(monto);

                let transaccion = TransaccionFiat::new(&usuario, monto, None);
                self.agregar_transaccion(Transaccion::IngresoDinero(transaccion));

                return true;
            }
            None => (),
        };
        false
    }

    fn comprar_cripto(
        &mut self,
        monto_fiat: f64,
        cripto: &'a Criptomoneda,
        usuario: &'a Usuario,
    ) -> bool {
        let cotizacion = self.get_cotizacion_cripto(cripto);

        let user = self.buscar_usuario(&usuario.dni);

        match user {
            Some(u) => {
                if u.esta_validado() && u.tiene_balance_suficiente(monto_fiat, None) {
                    u.compra_cripto(monto_fiat, &cripto.nombre, cotizacion);

                    let transaccion =
                        TransaccionCripto::new(usuario, cripto, monto_fiat, cotizacion);
                    self.agregar_transaccion(Transaccion::CompraCripto(transaccion));

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn vender_cripto(
        &mut self,
        monto_cripto: f64,
        cripto: &'a Criptomoneda,
        usuario: &'a Usuario,
    ) -> bool {
        let cotizacion = self.get_cotizacion_cripto(cripto);

        let user = self.buscar_usuario(&usuario.dni);

        match user {
            Some(u) => {
                if u.esta_validado()
                    && u.tiene_balance_suficiente(monto_cripto, Some(&cripto.nombre))
                {
                    u.compra_fiat(monto_cripto, &cripto.nombre, cotizacion);

                    let transaccion =
                        TransaccionCripto::new(usuario, cripto, monto_cripto, cotizacion);
                    self.agregar_transaccion(Transaccion::VentaCripto(transaccion));

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn retirar_cripto_a_blockchain(
        &mut self,
        monto: f64,
        cripto: &'a Criptomoneda,
        usuario: &'a Usuario,
        blockchain: &'a Blockchain,
    ) -> bool {
        if cripto.blockchains.contains(&blockchain) {
            let cotizacion = self.get_cotizacion_cripto(cripto);

            let user = self.buscar_usuario(&usuario.dni);

            match user {
                Some(u) => {
                    if u.esta_validado() && u.tiene_balance_suficiente(monto, Some(&cripto.nombre))
                    {
                        u.decrementar_balance_cripto(monto, &cripto.nombre);

                        let hash = blockchain.nombre.clone() + &rand::random::<u16>().to_string();
                        let transaccion = TransaccionRetiroRecepcion::new(
                            usuario,
                            blockchain,
                            Some(hash),
                            cripto,
                            monto,
                            cotizacion,
                        );
                        self.agregar_transaccion(Transaccion::RetiroCripto(transaccion));

                        return true;
                    }
                }
                None => (),
            }
        }

        false
    }

    fn recibir_cripto_de_blockchain(
        &mut self,
        monto: f64,
        cripto: &'a Criptomoneda,
        usuario: &'a Usuario,
        blockchain: &'a Blockchain,
    ) -> bool {
        if cripto.blockchains.contains(&blockchain) {
            let cotizacion = self.get_cotizacion_cripto(cripto);

            let user = self.buscar_usuario(&usuario.dni);

            match user {
                Some(u) => {
                    u.incrementar_balance_cripto(monto, &cripto.nombre);

                    let transaccion = TransaccionRetiroRecepcion::new(
                        usuario, blockchain, None, cripto, monto, cotizacion,
                    );
                    self.agregar_transaccion(Transaccion::RecepcionCripto(transaccion));

                    return true;
                }
                None => (),
            }
        }

        false
    }

    fn retirar_fiat(&mut self, monto: f64, usuario: &'a Usuario, medio_pago: MedioPago) -> bool {
        let user = self.buscar_usuario(&usuario.dni);

        match user {
            Some(u) => {
                if u.esta_validado() && u.tiene_balance_suficiente(monto, None) {
                    u.decrementar_balance_fiat(monto);

                    let transaccion = TransaccionFiat::new(usuario, monto, Some(medio_pago));
                    self.agregar_transaccion(Transaccion::RetiroDinero(transaccion));

                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn cripto_mas_ventas(&self) -> Option<&Criptomoneda> {
        let mut tabla = Sistema::get_tabla_cantidad_criptos();

        self.transacciones.iter().for_each(|t| match t {
            Transaccion::VentaCripto(info_tr) => {
                *tabla.get_mut(&info_tr.criptomoneda.nombre).unwrap() += 1 // La existencia de la criptomoneda es chequeada antes de la creacion de la transaccion
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
                *tabla.get_mut(&info_tr.criptomoneda.nombre).unwrap() += 1
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
                *tabla.get_mut(&info_tr.criptomoneda.nombre).unwrap() += volumen;
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
                *tabla.get_mut(&info_tr.criptomoneda.nombre).unwrap() += volumen;
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

impl<'a> Usuario {
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

impl<'a> PartialEq for Criptomoneda<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.nombre.eq(&other.nombre) && self.prefijo.eq(&other.prefijo)
    }
}

impl<'a> Hash for Criptomoneda<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.nombre.clone(), self.prefijo.clone()).hash(state);
    }
}

impl<'a> Criptomoneda<'a> {
    fn new(nombre: String) -> Criptomoneda<'a> {
        Criptomoneda {
            nombre: nombre.clone(),
            prefijo: nombre[..3].to_string(),
            blockchains: Vec::new(),
        }
    }

    fn agregar_blockchain(&mut self, blockchain: &'a Blockchain) {
        self.blockchains.push(blockchain)
    }
}

impl Blockchain {
    fn new(nombre: String, prefijo: String) -> Blockchain {
        Blockchain { nombre, prefijo }
    }
}

impl<'a> PartialEq for Transaccion<'a> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl<'a> TransaccionFiat<'a> {
    fn new(usuario: &'a Usuario, monto: f64, medio: Option<MedioPago>) -> TransaccionFiat<'a> {
        TransaccionFiat {
            usuario,
            fecha: Fecha::from(Local::now()),
            monto,
            medio,
        }
    }

    // No fue posible implementar trait Default para transacciones ya que poseen campos que son referencias
    fn default(usuario: &'a Usuario) -> TransaccionFiat<'a> {
        TransaccionFiat::new(usuario, 0.0, None)
    }
}

impl<'a> TransaccionCripto<'a> {
    fn new(
        usuario: &'a Usuario,
        criptomoneda: &'a Criptomoneda,
        monto: f64,
        cotizacion: f64,
    ) -> TransaccionCripto<'a> {
        TransaccionCripto {
            usuario,
            fecha: Fecha::from(Local::now()),
            criptomoneda,
            monto,
            cotizacion,
        }
    }

    fn default(usuario: &'a Usuario, criptomoneda: &'a Criptomoneda) -> TransaccionCripto<'a> {
        TransaccionCripto::new(usuario, criptomoneda, 0.0, 0.0)
    }
}

impl<'a> TransaccionRetiroRecepcion<'a> {
    fn new(
        usuario: &'a Usuario,
        blockchain: &'a Blockchain,
        hash: Option<String>,
        criptomoneda: &'a Criptomoneda,
        monto: f64,
        cotizacion: f64,
    ) -> TransaccionRetiroRecepcion<'a> {
        TransaccionRetiroRecepcion {
            usuario,
            fecha: Fecha::from(Local::now()),
            blockchain,
            hash,
            criptomoneda,
            monto,
            cotizacion,
        }
    }

    fn default(
        usuario: &'a Usuario,
        criptomoneda: &'a Criptomoneda,
        blockchain: &'a Blockchain,
    ) -> TransaccionRetiroRecepcion<'a> {
        TransaccionRetiroRecepcion::new(usuario, blockchain, None, criptomoneda, 0.0, 0.0)
    }
}

#[cfg(test)]
mod test {
    use chrono::format::StrftimeItems;

    use super::*;

    fn creacion_sistema<'a>() -> Sistema<'a> {
        // Creacion del sistema con 5 usuarios

        let mut sistema = Sistema::new();

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

        sistema.agregar_usuario(u1);
        sistema.agregar_usuario(u2);
        sistema.agregar_usuario(u3);
        sistema.agregar_usuario(u4);
        sistema.agregar_usuario(u5);

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

    fn creacion_criptos<'a>(blockchains: &'a Vec<Blockchain>) -> Vec<Criptomoneda<'a>> {
        // Creación de 4 criptomonedas y asociacion con blockchains

        let mut cripto1 = Criptomoneda::new("Bitcoin".to_string());
        cripto1.agregar_blockchain(&blockchains.get(0).unwrap());
        cripto1.agregar_blockchain(&blockchains.get(1).unwrap());

        let mut cripto2 = Criptomoneda::new("Ethereum".to_string());
        cripto2.agregar_blockchain(&blockchains.get(2).unwrap());
        cripto2.agregar_blockchain(&blockchains.get(3).unwrap());

        let mut cripto3 = Criptomoneda::new("BNB".to_string());
        cripto3.agregar_blockchain(&blockchains.get(4).unwrap());
        cripto3.agregar_blockchain(&blockchains.get(5).unwrap());

        let mut cripto4 = Criptomoneda::new("USDT".to_string());
        cripto4.agregar_blockchain(&blockchains.get(0).unwrap());
        cripto4.agregar_blockchain(&blockchains.get(1).unwrap());
        cripto4.agregar_blockchain(&blockchains.get(2).unwrap());
        cripto4.agregar_blockchain(&blockchains.get(3).unwrap());
        cripto4.agregar_blockchain(&blockchains.get(4).unwrap());
        cripto4.agregar_blockchain(&blockchains.get(5).unwrap());

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
        assert!(!sistema.agregar_usuario(rep_user));
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

        assert!(sistema.ingresar_dinero(499.99, &u1));
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

        assert!(!sistema.ingresar_dinero(55.25, &u2));

        // Chequea transaccion

        assert_eq!(sistema.get_transacciones().len(), 1);

        let transaccion = sistema.get_transacciones().first().unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::IngresoDinero(TransaccionFiat::default(&Default::default()))
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

        sistema.ingresar_dinero(1_000.0, &u1);
        sistema.ingresar_dinero(120_750.70, &u5);

        // Compro cripto para ambos (balances e identidades aptas). Evalúo balances y transacciones

        sistema.comprar_cripto(609.4, criptos.get(2).unwrap(), &u1); // BNB

        sistema.comprar_cripto(34_980.475, criptos.get(0).unwrap(), &u5); // Bitcoin
        sistema.comprar_cripto(39_265.0, criptos.get(1).unwrap(), &u5); // Ethereum

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
            &Transaccion::CompraCripto(TransaccionCripto::default(
                &Default::default(),
                &Default::default()
            ))
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

        sistema.ingresar_dinero(10_000.0, &u3);
        assert_eq!(
            sistema.buscar_usuario(&u3.dni).unwrap().balance_fiat,
            10_000.0
        );

        assert!(!sistema.comprar_cripto(50.0, criptos.get(3).unwrap(), &u3)); // USDT
        assert_eq!(
            sistema
                .buscar_usuario(&u3.dni)
                .unwrap()
                .get_balance_determinado(&"USDT".to_string())
                .unwrap(),
            &0.0
        );

        // Compra con usuario con saldo insuficiente

        assert!(!sistema.comprar_cripto(500.0, criptos.get(3).unwrap(), &u1)); // USDT
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

        assert!(!sistema.comprar_cripto(1.0, criptos.get(3).unwrap(), &u6));
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

        sistema.ingresar_dinero(100_000.0, &u1);
        sistema.ingresar_dinero(100_000.0, &u5);

        sistema.comprar_cripto(69_960.95, criptos.get(0).unwrap(), &u1); // Bitcoin
        sistema.comprar_cripto(30_000.0, criptos.get(3).unwrap(), &u1); // USDT

        sistema.comprar_cripto(39_265.0, criptos.get(1).unwrap(), &u5); // Ethereum
        sistema.comprar_cripto(6094.0, criptos.get(2).unwrap(), &u5); // BNB
        sistema.comprar_cripto(50_000.0, criptos.get(3).unwrap(), &u5); // USDT

        // Vendo cripto de usuarios validados y con balance

        sistema.vender_cripto(1.0, criptos.get(0).unwrap(), &u1); // Bitcoin
        sistema.vender_cripto(25_000.0, criptos.get(3).unwrap(), &u1); // USTD

        sistema.vender_cripto(5.0, criptos.get(1).unwrap(), &u5); // Ethereum

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
            &Transaccion::VentaCripto(TransaccionCripto::default(
                &Default::default(),
                &Default::default()
            ))
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

        assert!(!sistema.vender_cripto(10_000.0, criptos.get(3).unwrap(), &u3)); // USDT
        assert_eq!(sistema.buscar_usuario(&u3.dni).unwrap().balance_fiat, 0.0);

        // Compra con usuario con saldo insuficiente

        assert!(!sistema.vender_cripto(10_000.0, criptos.get(3).unwrap(), &u1)); // USDT, le quedan 5_000 USDT
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

        assert!(!sistema.vender_cripto(1.0, criptos.get(3).unwrap(), &u6));
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

        assert!(sistema.retirar_cripto_a_blockchain(
            3.5,
            criptos.get(0).unwrap(),
            &u1,
            blockchains.get(0).unwrap(),
        )); // Bitcoin, blockchain 0

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
            &Transaccion::RetiroCripto(TransaccionRetiroRecepcion::default(
                &Default::default(),
                &Default::default(),
                &Default::default()
            ))
        );

        if let Transaccion::RetiroCripto(t) = transaccion {
            assert_eq!(t.blockchain.prefijo, "BSC");
        }

        // Retiro usuario sin validacion

        assert!(!sistema.retirar_cripto_a_blockchain(
            10.0,
            criptos.get(1).unwrap(),
            &u5,
            blockchains.get(2).unwrap(),
        )); // Ethereum, blockchain 2

        // Retiro usuario con condiciones pero en blockchain erroneo

        assert!(!sistema.retirar_cripto_a_blockchain(
            0.5,
            criptos.get(0).unwrap(),
            &u1,
            blockchains.get(3).unwrap(),
        )); // Bitcoin, blockchain 3 (Bitcoin no opera con él)

        assert_eq!(
            sistema
                .buscar_usuario(&u1.dni)
                .unwrap()
                .get_balance_determinado(&"Bitcoin".to_string())
                .unwrap(),
            &1.5
        ); // Balance del usuario no ha cambiado

        // Retiro usuario sin balance suficiente

        assert!(!sistema.retirar_cripto_a_blockchain(
            2.0,
            criptos.get(0).unwrap(),
            &u1,
            blockchains.get(1).unwrap(),
        )); // Bitcoin, blockchain 1. Usuario solo tiene 1.5 Bitcoins

        // Retiro usuario no existente

        let u6 = Usuario::new(
            "Gaspar".to_string(),
            "Gonzalez".to_string(),
            "gaspi@hotmail.com".to_string(),
            "64928204".to_string(),
        );

        assert!(!sistema.retirar_cripto_a_blockchain(
            1.0,
            criptos.get(3).unwrap(),
            &u6,
            blockchains.get(3).unwrap(),
        )); // USDT, que opera con todos los blockchains
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

        assert!(sistema.recibir_cripto_de_blockchain(
            50.5,
            criptos.get(3).unwrap(),
            &u1,
            blockchains.get(2).unwrap(),
        )); // Recibe 50.5 USDT, que opera con todas las blockchains

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
            &Transaccion::RecepcionCripto(TransaccionRetiroRecepcion::default(
                &Default::default(),
                &Default::default(),
                &Default::default()
            ))
        ); // Compruebo que la transaccion se haya guardado

        if let Transaccion::RecepcionCripto(t) = transaccion {
            assert_eq!(t.usuario.dni, "45497524");
        } // Compruebo que el dato de la transaccion sea correcto

        // Recibir cripto con blockchain incorrecta

        assert!(!sistema.recibir_cripto_de_blockchain(
            20.5,
            criptos.get(2).unwrap(),
            &u1,
            blockchains.get(3).unwrap(),
        )); // BNB, opera con blockchains 4 y 5

        // Recibir cripto con usuario no existente

        let u6 = Usuario::new(
            "Gaspar".to_string(),
            "Gonzalez".to_string(),
            "gaspi@hotmail.com".to_string(),
            "64928204".to_string(),
        );

        assert!(!sistema.recibir_cripto_de_blockchain(
            5.0,
            criptos.get(0).unwrap(),
            &u6,
            blockchains.get(0).unwrap()
        )); // Bitcoin, opera con blockchains 0 y 1
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

        sistema.ingresar_dinero(5000.0, &u1);

        sistema.buscar_usuario(&u5.dni).unwrap().balance_fiat += 3500.0; // Introduzco fiat artificialmente a usuario no validado

        // Retiro usuario validado y con balance suficiente

        assert!(sistema.retirar_fiat(4000.0, &u1, MedioPago::Transferencia));
        assert_eq!(
            sistema.buscar_usuario(&u1.dni).unwrap().balance_fiat,
            1000.0
        );

        let transaccion = sistema.get_transacciones().get(1).unwrap();
        assert_eq!(
            transaccion,
            &Transaccion::RetiroDinero(TransaccionFiat::default(&Default::default()))
        );

        if let Transaccion::RetiroDinero(t) = transaccion {
            assert_eq!(t.medio.as_ref().unwrap(), &MedioPago::Transferencia);
        }

        // Retiro usuario sin balance suficiente

        assert!(!sistema.retirar_fiat(1500.0, &u1, MedioPago::MercadoPago));

        // Retiro usuario sin validar

        assert!(!sistema.retirar_fiat(500.0, &u5, MedioPago::MercadoPago));
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

        assert!(!sistema.retirar_fiat(5.0, &u6, MedioPago::Transferencia));
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

        sistema.ingresar_dinero(100_000.0, &u1);
        sistema.ingresar_dinero(100_000.0, &u5);

        // Usuario 1 - Compra cripto
        sistema.comprar_cripto(50_000.0, criptos.get(0).unwrap(), &u1); // Bitcoin - 0.71
        sistema.comprar_cripto(30_000.0, criptos.get(1).unwrap(), &u1); // Ethereum - 7.64

        // Usuario 2 - Compra cripto
        sistema.comprar_cripto(80_000.0, criptos.get(0).unwrap(), &u5); // Bitcoin - 1.14
        sistema.comprar_cripto(10_000.0, criptos.get(3).unwrap(), &u5); // USDT - 10_000
        sistema.comprar_cripto(5_000.0, criptos.get(1).unwrap(), &u5); // Ethereum - 1.27

        // Usuario 1 - Venta cripto
        sistema.vender_cripto(5.0, criptos.get(1).unwrap(), &u1); // Ethereum

        // Usuario 2 - Venta cripto
        sistema.vender_cripto(1.0, criptos.get(1).unwrap(), &u5); // Ethereum
        sistema.vender_cripto(1.0, criptos.get(0).unwrap(), &u5); // Bitcoin

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
        let mut sistema = Sistema::new();

        let u = Usuario::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "example@gmail.com".to_string(),
            "45786392".to_string(),
        );
        let mut c = Criptomoneda::new("USDT".to_string());
        let b = Blockchain::new("Binance Smart Chain".to_string(), "BSC".to_string());
        c.agregar_blockchain(&b);

        assert!(!sistema.ingresar_dinero(10.0, &u));

        assert!(!sistema.comprar_cripto(10.0, &c, &u));

        assert!(!sistema.vender_cripto(10.0, &c, &u));

        assert!(!sistema.retirar_cripto_a_blockchain(10.0, &c, &u, &b));

        assert!(!sistema.recibir_cripto_de_blockchain(10.0, &c, &u, &b));

        assert!(!sistema.retirar_fiat(10.0, &u, MedioPago::MercadoPago));

        assert!(sistema.cripto_mas_ventas().is_none());

        assert!(sistema.cripto_mas_compras().is_none());

        assert!(sistema.cripto_mayor_volumen_venta().is_none());

        assert!(sistema.cripto_mayor_volumen_compra().is_none());
    }
}
