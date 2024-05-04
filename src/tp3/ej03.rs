use std::collections::BTreeMap;

struct Fecha {
    dia: u32,
    mes: u32,
    anio: u32,
}

impl Fecha {
    fn new(dia: u32, mes: u32, anio: u32) -> Fecha {
        Fecha { dia, mes, anio }
    }

    fn get_dias_mes(mes: u32) -> u32 {
        let dias_mes = BTreeMap::from([
            (1, 31),
            (2, 28),
            (3, 31),
            (4, 30),
            (5, 31),
            (6, 30),
            (7, 31),
            (8, 31),
            (9, 30),
            (10, 31),
            (11, 30),
            (12, 31),
        ]);

        dias_mes[&mes]
    }

    fn es_fecha_valida(&self) -> bool {
        match self {
            f if f.mes >= 1 && f.mes <= 12 => match self {
                f if f.dia >= 1 && f.dia <= Fecha::get_dias_mes(f.mes) => true,
                f if f.es_bisiesto() && f.mes == 2 => f.dia >= 1 && f.dia <= 29,
                _ => false,
            },

            _ => false,
        }
    }

    // Un año es bisiesto si es divisible por 4. En caso de ser centenario (divisible por 100),
    //ese año debe ser divisible por 400 para ser bisiesto
    fn es_bisiesto(&self) -> bool {
        let anio = self.anio;
        if anio % 4 == 0 {
            if anio % 100 == 0 {
                match anio {
                    a if a % 400 == 0 => return true,
                    _ => return false,
                }
            } else {
                return true;
            }
        }
        false
    }

    fn sumar_dias(&mut self, dias: u32) {
        self.dia += dias;

        while !self.es_fecha_valida() {
            if self.mes <= 12 {
                match self {
                    ref f if f.mes == 2 && f.es_bisiesto() => self.dia -= 29,
                    _ => self.dia -= Fecha::get_dias_mes(self.mes),
                };
                self.mes += 1;
            } else {
                self.anio += 1;
                self.mes = 1;
            }
        }
    }

    fn restar_dias(&mut self, dias: u32) {
        let mut dia = self.dia as i32;
        dia -= dias as i32;

        while dia < 1 {
            if self.mes > 1 {
                self.mes -= 1;
                match self {
                    ref f if f.mes == 2 && f.es_bisiesto() => dia += 29,
                    _ => dia += Fecha::get_dias_mes(self.mes) as i32,
                }
            } else {
                dia += 31; // enero
                self.anio -= 1;
                self.mes = 12;
            }
        }

        self.dia = dia as u32;
    }

    fn es_mayor(&self, fecha: &Fecha) -> bool {
        match self {
            ref f if f.anio > fecha.anio => true,
            ref f if f.anio == fecha.anio && f.mes > fecha.mes => true,
            ref f if f.anio == fecha.anio && f.mes == fecha.mes && f.dia > fecha.dia => true,
            _ => false,
        }
    }
}

#[test]
fn test_fecha_valida() {
    let f1 = Fecha::new(18, 8, 2077);

    assert!(f1.es_fecha_valida());
}

#[test]
fn test_fecha_valida_bisiesto() {
    let f1 = Fecha::new(29, 2, 2024);

    assert!(f1.es_bisiesto());
    assert!(f1.es_fecha_valida());
}

#[test]
fn test_fecha_erronea() {
    let f1 = Fecha::new(29, 2, 2021);

    assert!(!f1.es_bisiesto());
    assert!(!f1.es_fecha_valida());
}

#[test]
fn test_sumar_dias() {
    let mut f1 = Fecha::new(1, 1, 2024);
    let f2 = Fecha::new(28, 1, 2048);
    f1.sumar_dias(8793);

    assert_eq!(f1.dia, f2.dia);
    assert_eq!(f1.mes, f2.mes);
    assert_eq!(f1.anio, f2.anio);
}

#[test]
fn test_restar_dias() {
    let mut f1 = Fecha::new(28, 1, 2048);
    let f2 = Fecha::new(1, 1, 2024);
    f1.restar_dias(8793);

    assert_eq!(f1.dia, f2.dia);
    assert_eq!(f1.mes, f2.mes);
    assert_eq!(f1.anio, f2.anio);
}

#[test]
fn test_comparar_fechas() {
    let f1 = Fecha::new(1, 1, 2024);
    let f2 = Fecha::new(28, 1, 2024);

    assert!(f1.es_bisiesto());

    assert!(f1.es_fecha_valida());
    assert!((f2.es_fecha_valida()));

    assert!(f2.es_mayor(&f1));
}

#[test]
fn test_operaciones_fecha() {
    let mut f1 = Fecha::new(1, 1, 2024);
    let mut f2 = Fecha::new(1, 1, 2024);

    assert!(f1.es_fecha_valida());

    assert!((f1.es_bisiesto()));

    f1.sumar_dias(7643);
    assert_eq!(f1.dia, 4);
    assert_eq!(f1.mes, 12);
    assert_eq!(f1.anio, 2044);

    assert!(f1.es_fecha_valida());
    assert!(f1.es_bisiesto());

    f2.restar_dias(6427);
    assert_eq!(f2.dia, 28);
    assert_eq!(f2.mes, 5);
    assert_eq!(f2.anio, 2006);

    assert!(f2.es_fecha_valida());
    assert!(!f2.es_bisiesto());

    assert!(f1.es_mayor(&f2));
}
