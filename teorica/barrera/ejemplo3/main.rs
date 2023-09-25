/*
    A diferencia de los Locks, los semaforos no son exclusivos por thread
    Es decir, se puede liberar recursos desde otro thread
*/

use std::sync::Arc;
use std_semaphore::Semaphore;

fn main() {
    let sem = Arc::new(Semaphore::new(0));
    let c_sem = sem.clone();

    let _h = std::thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        c_sem.release(); // Se liberan los recursos
        println!("Soy el hijo release");
    });

    println!("Soy el padre: Voy a obtener el semaforo");

    // En este punto queda bloqueado hasta que se liberen los recursos
    let _guard = sem.access();

    println!("Soy el padre: Semaforo adquirido");
}
