// Pag 125: Algorithm 6.12: Dining philosophers (third attempt)

/*
Another solution that is free from starvation is an asymmetric
algorithm, which has the first four philosophers execute the original
solucion (without the rooms), but the fifth philosopher waits first for
the right fork and then for the left fork.

*/

use std::{sync::Arc, thread, time::Duration, vec};
use std_semaphore::Semaphore;

fn eat(i: usize) {
    println!("Filosofo {i} esta comiendo");
    thread::sleep(Duration::from_millis(1000));
}

fn think(_i: usize) {
    //println!("Filosofo {i} esta pensando");
    thread::sleep(Duration::from_millis(1000));
}

fn main() {
    const NUM_FILOSOFOS: usize = 5;
    let mut handles = vec![];

    // Los tenedores son semaforos que indican si estan usados o no
    // tenedor[i] es el tenedor del filosofo i e indica si esta siendo usado o no
    let tenedores = (0..NUM_FILOSOFOS)
        .map(|_| Arc::new(Semaphore::new(1)))
        .collect::<Vec<_>>();

    for i in 0..NUM_FILOSOFOS - 1 {
        let arc_tenedores = tenedores.clone();

        handles.push(thread::spawn(move || {
            loop {
                think(i);

                arc_tenedores[i].acquire(); // wait(fork[i]) -> Tenedor izquierdo
                arc_tenedores[(i + 1) % 5].acquire(); // wait(fork[i+1]) == wait(fork[(i+1) % 5]) -> Tenedor derecho

                eat(i);

                arc_tenedores[i].release(); // signal(fork[i]) -> Tenedor izquierdo
                arc_tenedores[(i + 1) % 5].release(); // signal(fork[i+1]) = wait(fork[(i+1) % 5]) -> Tenedor derecho
            }
        }));

        // Ultimo filosofo. Toma los tenedores en el orden inverso
        let arc_tenedores = tenedores.clone();

        handles.push(thread::spawn(move || {
            loop {
                think(NUM_FILOSOFOS - 1);

                arc_tenedores[0].acquire(); // wait(fork[i+1]) == wait(fork[(i+1) % 5]) Tenedor derecho
                arc_tenedores[NUM_FILOSOFOS - 1].acquire(); // wait(fork[i]) Tenedor izquierdo

                eat(NUM_FILOSOFOS - 1);

                arc_tenedores[0].release(); // signal(fork[i+1]) = wait(fork[(i+1) % 5]) -> Tenedor derecho
                arc_tenedores[NUM_FILOSOFOS - 1].release(); // signal(fork[i]) -> Tenedor izquierdo
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
