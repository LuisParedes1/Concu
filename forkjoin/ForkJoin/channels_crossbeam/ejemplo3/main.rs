use crossbeam_channel::unbounded;
use std::thread;
use std::time::Duration;

fn main() {
    let (s1, r1) = unbounded();
    let (s2, r2) = (s1.clone(), r1.clone());

    let h = thread::spawn(move || {
        println!("Soy el hijo: envio!");
        s2.send("- Hijo envia -").unwrap();
        thread::sleep(Duration::from_millis(5_000));
        println!("Hijo recibe: {}", r2.recv().unwrap());
    });

    println!("Padre recibe: {}", r1.recv().unwrap());

    println!("Soy el padre: envio!");
    s1.send("- Padre envia -").unwrap();

    h.join().unwrap();
}
