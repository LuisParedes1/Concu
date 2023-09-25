// Ejemplo de Barrera
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let mut handles = Vec::with_capacity(10);

    // Tengo 10 recursos
    let barrier = Arc::new(Barrier::new(10));

    for i in 0..10 {
        let c = barrier.clone();
        handles.push(thread::spawn(move || {
            println!("Antes del wait");
            let barrier_wait_res = c.wait(); // Todos se bloquean en este punto hasta
                                             // ser liberados por el thread lider
            println!("Despues del wait");
            println!("Soy el hilo {}: {:?}", i, barrier_wait_res.is_leader());
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
