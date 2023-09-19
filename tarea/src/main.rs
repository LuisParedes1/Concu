/* Realizar un pequeño idle game

* Algunas entidades "extraeran" (generaran aleatoriamente) oro
* Algunas entidades podran convertir oro en recursos (a gusto)
* Otras entidades podran convertir combinaciones de recursos en + oro
* Otras entidades podran solamente consumir recursos
* Periodicamente se reporta por pantalla el nivel de recursos y oro


Modelaremos cada entidad en un thread independiente

Las entidades son
- Pasto
- Maiz
- Madera
- Carbon
*/

use rand::{thread_rng, Rng};

use std::{
    sync::{Arc, RwLock},
    thread::{self, JoinHandle},
    time::Duration,
};

use tarea::recursos::generar_recursos;

mod recursos;

fn main() {
    let recursos = generar_recursos();

    let mut handles = vec![];

    // Algunas entidades "extraeran" (generaran aleatoriamente) oro
    let rec0 = recursos.clone();
    handles.push(thread::spawn(move || loop {
        if let Ok(mut recursos) = rec0.write() {
            recursos[0].cantidad += 1;
        }
        thread::yield_now();
    }));

    // Algunas entidades podran convertir oro en recursos (a gusto)
    let rec1 = recursos.clone();
    handles.push(thread::spawn(move || loop {
        if let Ok(mut recursos) = rec1.write() {
            if recursos[0].cantidad > 0 {
                recursos[0].cantidad -= 1;
                recursos[thread_rng().gen_range(1, 5)].cantidad += 1;
            }
        }
        thread::yield_now();
    }));

    // Otras entidades podran convertir combinaciones de recursos en + oro
    let rec2 = recursos.clone();
    handles.push(thread::spawn(move || loop {
        if let Ok(mut recursos) = rec2.write() {
            let mut i = 0;
            let mut j = 0;

            while i == j {
                i = thread_rng().gen_range(1, 5);
                j = thread_rng().gen_range(1, 5);
            }

            if recursos[i].cantidad > 0 && recursos[j].cantidad > 0 {
                recursos[i].cantidad -= 1;
                recursos[j].cantidad -= 1;
                recursos[0].cantidad += 1;
            }
        }
        thread::yield_now();
    }));

    // Otras entidades podran solamente consumir recursos
    let rec3 = recursos.clone();
    handles.push(thread::spawn(move || loop {
        if let Ok(mut recursos) = rec3.write() {
            let indice_rec = thread_rng().gen_range(1, 5);

            if recursos[indice_rec].cantidad > 0 {
                recursos[indice_rec].cantidad -= 1;
            }
        }
        thread::yield_now();
    }));

    // Periodicamente se reporta por pantalla el nivel de recursos y oro
    let rec4 = recursos.clone();
    handles.push(thread::spawn(move || loop {
        if let Ok(recursos) = rec4.read() {
            for rec in recursos.iter() {
                println!("{}: {}", rec.nombre, rec.cantidad);
            }
        }
        thread::sleep(Duration::from_millis(10000));
    }));

    for handle in handles {
        handle.join().unwrap();
    }
}
