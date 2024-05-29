use std::{collections::HashMap, fmt::Debug, hash::Hash};

use crate::tp3::ej03::Fecha;

const DESCUENTO_SUSCRIPCION: f64 = 0.15;

struct Producto {
    nombre: String,
    categoria: Categorias,
    precio_base: f64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Categorias {
    Limpieza,
    Comestibles,
    Indumentaria,
    Otros,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Persona {
    nombre: String,
    apellido: String,
    direccion: String,
    dni: String,
}

#[derive(Debug, PartialEq, Clone)]
struct Vendedor {
    datos: Persona,
    nro_legajo: u16,
    antiguedad: i32,
    salario: f64,
}

#[derive(Debug, PartialEq, Clone)]
struct Cliente {
    datos: Persona,
    suscripcion: bool,
    email: Option<String>,
}

struct Venta {
    vendedor: Vendedor,
    cliente: Cliente,
    medio_pago: MediosDePago,
    fecha: Fecha,
    productos: Vec<Producto>,
}

enum MediosDePago {
    Credito,
    Debito,
    Transferencia,
    Efectivo,
}

struct SistemaVentas {
    ventas: Vec<Venta>,
    descuentos_categorias: HashMap<Categorias, f64>,
}

impl Producto {
    fn new(nombre: String, categoria: Categorias, precio_base: f64) -> Producto {
        Producto {
            nombre,
            categoria,
            precio_base,
        }
    }

    fn calcular_precio(&self, descuento: Option<&f64>) -> f64 {
        match descuento {
            Some(descuento) => self.precio_base - (self.precio_base * descuento),
            None => self.precio_base,
        }
    }
}

impl Categorias {
    fn get_tabla_categorias() -> HashMap<Categorias, u32> {
        HashMap::from([
            (Categorias::Limpieza, 0),
            (Categorias::Comestibles, 0),
            (Categorias::Indumentaria, 0),
            (Categorias::Otros, 0),
        ])
    }
}

impl Persona {
    fn new(nombre: String, apellido: String, direccion: String, dni: String) -> Persona {
        Persona {
            nombre,
            apellido,
            direccion,
            dni,
        }
    }

    fn get_dni(&self) -> &String {
        &self.dni
    }
}

impl Vendedor {
    fn new(datos: Persona, nro_legajo: u16, antiguedad: i32, salario: f64) -> Vendedor {
        Vendedor {
            datos,
            nro_legajo,
            antiguedad,
            salario,
        }
    }

    fn get_numero_legajo(&self) -> u16 {
        self.nro_legajo
    }

    fn get_datos(&self) -> &Persona {
        &self.datos
    }
}

impl Cliente {
    fn new(datos: Persona, email: Option<String>) -> Cliente {
        let suscripcion = match email {
            Some(_) => true,
            None => false,
        };

        Cliente {
            datos,
            suscripcion,
            email,
        }
    }
}

impl Venta {
    fn new(vendedor: Vendedor, cliente: Cliente, medio_pago: MediosDePago, fecha: Fecha) -> Venta {
        Venta {
            vendedor,
            cliente,
            medio_pago,
            fecha,
            productos: Vec::new(),
        }
    }

    fn get_vendedor(&self) -> &Vendedor {
        &self.vendedor
    }

    fn get_cliente(&self) -> &Cliente {
        &self.cliente
    }

    fn get_medio_pago(&self) -> &MediosDePago {
        &self.medio_pago
    }

    fn get_fecha(&self) -> &Fecha {
        &self.fecha
    }

    fn from(&mut self, productos: Vec<Producto>) {
        self.productos = productos;
    }

    fn agregar_producto(&mut self, producto: Producto) {
        self.productos.push(producto);
    }

    fn calcular_precio_final(&self, tabla_descuentos: &HashMap<Categorias, f64>) -> f64 {
        let precio = self
            .productos
            .iter()
            .map(|p| p.calcular_precio(tabla_descuentos.get(&p.categoria)))
            .sum();

        match self.cliente.suscripcion {
            true => precio - (precio * DESCUENTO_SUSCRIPCION),
            false => precio,
        }
    }
}

impl SistemaVentas {
    fn new() -> SistemaVentas {
        let tabla_descuentos = HashMap::from([
            (Categorias::Limpieza, 0.2),
            (Categorias::Comestibles, 0.1),
            (Categorias::Indumentaria, 0.3),
        ]);

        SistemaVentas {
            ventas: Vec::new(),
            descuentos_categorias: tabla_descuentos,
        }
    }

    fn get_tabla_descuentos(&self) -> &HashMap<Categorias, f64> {
        &self.descuentos_categorias
    }

    fn get_ventas(&self) -> &Vec<Venta> {
        &self.ventas
    }

    // Analiza si el vendedor esta registrado en alguna venta (por su legajo). Si lo está, se cerciora que los dni de ambos sean iguales
    // Evita duplicaciones de nro de legajo
    fn es_vendedor_valido(&self, vendedor: &Vendedor) -> bool {
        let vendedor_buscado =
            self.ventas
                .iter()
                .find_map(|v| match v.get_vendedor().get_numero_legajo() {
                    nro_legajo if nro_legajo == vendedor.get_numero_legajo() => {
                        Some(v.get_vendedor())
                    }
                    _ => None,
                });

        match vendedor_buscado {
            Some(v) => match v.get_datos().get_dni() {
                dni if dni == vendedor.get_datos().get_dni() => true,
                _ => false,
            },
            None => true,
        }
    }

    fn agregar_venta(&mut self, venta: Venta) -> bool {
        match self.es_vendedor_valido(venta.get_vendedor()) {
            true => self.ventas.push(venta),
            false => return false,
        }

        true
    }

    fn reporte_ventas_categoria(&self) -> Vec<(Categorias, u32)> {
        let mut ventas_categorias = Categorias::get_tabla_categorias();

        self.ventas.iter().for_each(|v| {
            v.productos
                .iter()
                .for_each(|p| *ventas_categorias.get_mut(&p.categoria).unwrap() += 1)
        });

        ventas_categorias.into_iter().collect()
    }

    fn reporte_ventas_vendedor(&self) -> Vec<(u16, u32)> {
        let mut ventas_vendedor = HashMap::new();

        self.ventas.iter().for_each(|v| {
            match ventas_vendedor.get_mut(&v.get_vendedor().get_numero_legajo()) {
                Some(cantidad) => *cantidad += 1,
                None => {
                    ventas_vendedor.insert(v.get_vendedor().get_numero_legajo(), 1);
                }
            }
        });

        ventas_vendedor.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn creacion_sistema() -> SistemaVentas {
        let mut sistema_ventas = SistemaVentas::new();

        // Vendedores

        let p1 = Persona::new(
            "Nahuel".to_string(),
            "Luna".to_string(),
            "155".to_string(),
            "38746293".to_string(),
        );

        let p2 = Persona::new(
            "Gaspar".to_string(),
            "Perez".to_string(),
            "520".to_string(),
            "43742273".to_string(),
        );

        let p3 = Persona::new(
            "Pedro".to_string(),
            "Simons".to_string(),
            "Mitre".to_string(),
            "32684021".to_string(),
        );

        // Clientes

        let p4 = Persona::new(
            "Valentin".to_string(),
            "Lemans".to_string(),
            "Mitre".to_string(),
            "51745221".to_string(),
        );

        let p5 = Persona::new(
            "Juan".to_string(),
            "Hernandez".to_string(),
            "Avellaneda".to_string(),
            "28746323".to_string(),
        );

        let p6 = Persona::new(
            "Leo".to_string(),
            "Messi".to_string(),
            "Barcelona".to_string(),
            "13568452".to_string(),
        );

        // Crear vendedores, clientes y ventas

        let v1 = Vendedor::new(p1, 4, 6, 555.5);
        let v2 = Vendedor::new(p2, 2, 3, 1750.5);
        let v3 = Vendedor::new(p3, 8, 3, 777.77);

        let c1 = Cliente::new(p4, None);
        let c2 = Cliente::new(p5, Some("example@hotmail.com".to_string()));
        let c3 = Cliente::new(p6, Some("test@gmail.com".to_string()));

        let mut venta1 = Venta::new(
            v1.clone(),
            c1,
            MediosDePago::Efectivo,
            Fecha::new(25, 5, 2024),
        );
        let mut venta2 = Venta::new(
            v2,
            c2.clone(),
            MediosDePago::Transferencia,
            Fecha::new(26, 4, 2024),
        );
        let mut venta3 = Venta::new(v1, c3, MediosDePago::Debito, Fecha::new(29, 2, 2024));
        let mut venta4 = Venta::new(v3, c2, MediosDePago::Debito, Fecha::new(8, 6, 2024));

        // Agregar productos a las ventas

        venta1.agregar_producto(Producto::new(
            "Milanesa".to_string(),
            Categorias::Comestibles,
            500.0,
        ));
        venta1.agregar_producto(Producto::new(
            "Lavandina".to_string(),
            Categorias::Limpieza,
            350.25,
        ));
        venta1.agregar_producto(Producto::new(
            "Pantalon".to_string(),
            Categorias::Indumentaria,
            4999.99,
        ));

        venta2.agregar_producto(Producto::new(
            "Escoba".to_string(),
            Categorias::Otros,
            255.0,
        ));
        venta2.agregar_producto(Producto::new(
            "Fideos".to_string(),
            Categorias::Comestibles,
            150.5,
        ));

        venta3.agregar_producto(Producto::new(
            "Escritorio".to_string(),
            Categorias::Otros,
            8750.5,
        ));
        venta4.agregar_producto(Producto::new(
            "Notebook".to_string(),
            Categorias::Otros,
            99999.99,
        ));
        venta4.agregar_producto(Producto::new(
            "Aspiradora de mano".to_string(),
            Categorias::Limpieza,
            899.5,
        ));

        // Agregar ventas al sistema

        sistema_ventas.agregar_venta(venta1);
        sistema_ventas.agregar_venta(venta2);
        sistema_ventas.agregar_venta(venta3);
        sistema_ventas.agregar_venta(venta4);

        sistema_ventas
    }

    #[test]
    fn test_datos_del_sistema() {
        let mut sistema_ventas = creacion_sistema();

        // Comprueba cantidad de ventas y que un dato de la venta sea el almacenado

        assert_eq!(sistema_ventas.get_ventas().len(), 4);
        assert_eq!(
            sistema_ventas
                .get_ventas()
                .first()
                .unwrap()
                .get_vendedor()
                .get_numero_legajo(),
            4
        );

        // Intenta agregar venta con vendedor cuyo nro de legajo ya ha sido registrado por otro

        let vendedor = Vendedor::new(
            Persona::new(
                "Pedro".to_string(),
                "Hernandez".to_string(),
                "160".to_string(),
                "45496278".to_string(),
            ),
            4,
            1,
            100.6,
        );
        let cliente = sistema_ventas
            .get_ventas()
            .first()
            .unwrap()
            .get_cliente()
            .clone();

        assert!(!sistema_ventas.agregar_venta(Venta::new(
            vendedor,
            cliente.clone(),
            MediosDePago::Efectivo,
            Fecha::new(28, 2, 2023)
        )));

        // Agrega una venta valida (con un vendedor registrado)

        let vendedor2 = sistema_ventas
            .get_ventas()
            .get(1)
            .unwrap()
            .get_vendedor()
            .clone();

        assert!(sistema_ventas.agregar_venta(Venta::new(
            vendedor2,
            cliente,
            MediosDePago::Credito,
            Fecha::new(1, 1, 2024)
        )));

        assert_eq!(sistema_ventas.get_ventas().len(), 5);
    }

    #[test]
    fn test_calcular_precio() {
        let mut sistema = creacion_sistema();

        let venta1 = sistema.get_ventas().first().unwrap();
        let venta2 = sistema.get_ventas().get(1).unwrap();
        let venta3 = sistema.get_ventas().get(2).unwrap();
        let venta4 = sistema.get_ventas().last().unwrap();

        // Compruebo precios de todas las ventas

        assert_eq!(
            venta1.calcular_precio_final(sistema.get_tabla_descuentos()),
            4230.193
        );

        assert_eq!(
            venta2.calcular_precio_final(sistema.get_tabla_descuentos()),
            331.8825
        );

        assert_eq!(
            venta3.calcular_precio_final(sistema.get_tabla_descuentos()),
            7437.925
        );

        assert_eq!(
            venta4.calcular_precio_final(sistema.get_tabla_descuentos()),
            85_611.6515
        );

        // Intento calcular precio de venta vacia

        let venta_vacia = sistema.ventas.last_mut().unwrap();
        venta_vacia.productos.clear();

        assert_eq!(
            venta_vacia.calcular_precio_final(&sistema.descuentos_categorias),
            0.0
        );
    }

    #[test]
    fn test_reporte_ventas_categorias() {
        let sistema = creacion_sistema();

        let reporte = sistema.reporte_ventas_categoria();

        // Debido a que converti de HashMap a Vec, desconozco el orden del vector, por lo que busco cada categoria

        let categoria_otros = reporte.iter().find(|r| r.0 == Categorias::Otros).unwrap();
        let categoria_comestible = reporte
            .iter()
            .find(|r| r.0 == Categorias::Comestibles)
            .unwrap();
        let categoria_indumentaria = reporte
            .iter()
            .find(|r| r.0 == Categorias::Indumentaria)
            .unwrap();
        let categoria_limpieza = reporte
            .iter()
            .find(|r| r.0 == Categorias::Limpieza)
            .unwrap();

        // Compruebo las cantidades de productos vendidos por categoria

        assert_eq!(categoria_otros.1, 3);

        assert_eq!(categoria_comestible.1, 2);

        assert_eq!(categoria_indumentaria.1, 1);

        assert_eq!(categoria_limpieza.1, 2);
    }

    #[test]
    fn test_reporte_ventas_vendedores() {
        let sistema = creacion_sistema();

        // Creo el reporte y busco cada vendedor

        let reporte = sistema.reporte_ventas_vendedor();

        let v1 = reporte.iter().find(|legajo| legajo.0 == 4).unwrap();
        let v2 = reporte.iter().find(|legajo| legajo.0 == 2).unwrap();
        let v3 = reporte.iter().find(|legajo| legajo.0 == 8).unwrap();

        // Compruebo que la cantidad almacenada se corresponda con las ventas del vendedor

        assert_eq!(v1.1, 2);

        assert_eq!(v2.1, 1);

        assert_eq!(v3.1, 1);
    }
}
