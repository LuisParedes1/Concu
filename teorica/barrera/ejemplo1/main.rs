/*
    Acquire = wait
    Release = signal
*/

use std_semaphore::Semaphore;

fn main() {
    let sem = Semaphore::new(1);

    sem.acquire(); // wait

    println!("Semaforo Adquirido");

    sem.release(); // signal

    println!("Semaaforo Liberado");
}
