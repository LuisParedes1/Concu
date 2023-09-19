use std::sync::{Arc, RwLock};

pub struct Recurso {
    pub nombre: String,
    pub cantidad: i32,
}

pub fn generar_recursos() -> Arc<RwLock<Vec<Recurso>>> {
    Arc::new(RwLock::new(vec![
        Recurso {
            nombre: "oro".to_string(),
            cantidad: 0,
        },
        Recurso {
            nombre: "pasto".to_string(),
            cantidad: 0,
        },
        Recurso {
            nombre: "maiz".to_string(),
            cantidad: 0,
        },
        Recurso {
            nombre: "madera".to_string(),
            cantidad: 0,
        },
        Recurso {
            nombre: "carbon".to_string(),
            cantidad: 0,
        },
    ]))
}
