use std::thread;
use std::time::Duration;

fn main() {
    println!("[PADRE] spawneo hijo");
    let join_handle = thread::spawn(move || {
        println!("[HIJO] spawnie");
        thread::sleep(Duration::from_secs(2));
        println!("[HIJO] me desperté")
    });
    println!("[PADRE] esperando hijo");
    join_handle.join();
    println!("[PADRE] terminó");
}