use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(1);

    {
        // Creo un Lock Exclusivo
        let mut data_w = lock.write().expect("failed");
        *data_w = 2; // escribo
    } // Al salir dataw del scope y hacerse drop entonces se hace unlock()

    let data_r = lock.read().expect("failed");

    println!("El valor encontrado es {}", *data_r);
}
