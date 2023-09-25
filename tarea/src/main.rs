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

    // Para evitar deadlocks, se impone que solo pueden haber NUM_FILOSOFOS - 1 filosofos en la habitacion
    let habitaciones = Arc::new(Semaphore::new(NUM_FILOSOFOS as isize - 1));

    // Los tenedores son semaforos que indican si estan usados o no
    // tenedor[i] es el tenedor del filosofo i e indica si esta siendo usado o no
    let tenedores = (0..NUM_FILOSOFOS)
        .map(|_| Arc::new(Semaphore::new(1)))
        .collect::<Vec<_>>();

    for i in 0..NUM_FILOSOFOS {
        let arc_tenedores = tenedores.clone();
        let arc_habitaciones = habitaciones.clone();

        handles.push(thread::spawn(move || {
            loop {
                think(i);

                arc_habitaciones.acquire(); // wait(room)

                arc_tenedores[i].acquire(); // wait(fork[i])
                arc_tenedores[(i + 1) % 5].acquire(); // wait(fork[i+1]) == wait(fork[(i+1) % 5])

                eat(i);

                arc_tenedores[i].release(); // signal(fork[i])
                arc_tenedores[(i + 1) % 5].release(); // signal(fork[i+1]) = wait(fork[(i+1) % 5])

                arc_habitaciones.release(); // signal(room)
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
