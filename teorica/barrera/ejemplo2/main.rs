/*
    Access = accquire + release automatico
*/

use std_semaphore::Semaphore;

fn main() {
    let sem = Semaphore::new(1);

    let a = 10;

    {
        let _guard = sem.access();
        println!("Semafoto Adquirido");
        println!("a = {}", a);
    }

    println!("Semaforo Liberado");
}
