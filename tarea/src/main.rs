use core::num;
use std::{sync::Arc, thread, vec};
use std_semaphore::Semaphore;

fn eat() {
    println!("Filosofo esta comiendo");
    thread::sleep_ms(1000);
}

fn think() {
    println!("Filosofo esta pensando");
    thread::sleep_ms(1000);
}

fn main() {
    const NUM_FILOSOFOS: usize = 5;
    let mut handles = vec![];

    // Los tenedores son semaforos que indican si estan usados o no
    let tenedores = (0..NUM_FILOSOFOS)
        .map(|_| Arc::new(Semaphore::new(1)))
        .collect::<Vec<_>>();

    for i in 0..NUM_FILOSOFOS {
        let arc_tenedores = tenedores.clone();
        handles.push(thread::spawn(move || {
            loop {
                think();

                arc_tenedores[i].acquire(); // wait[i]
                arc_tenedores[(i + 1) % 5].acquire(); // wait[i+1] == wait[(i+1) % 5]]

                eat();

                arc_tenedores[i].release(); // signal[i]
                arc_tenedores[(i + 1) % 5].release(); // signal[i+1] = wait[(i+1) % 5]]
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
