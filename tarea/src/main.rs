use std::collections::HashMap;
use std::{env, thread};
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};

use rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use std::path::PathBuf;
use std::time::{Instant, Duration};



// Calcula la cantidad total de vistas por canal (sumando todos los archivos .csv)
fn vistas_por_canal() -> HashMap<String, i64>{


    let result = read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/data")).unwrap() // Veo los archivos dentro del directorio
        
        .map(|d| d.unwrap().path())  // Cada archvio obtengo su path
        
        .flat_map(|path| { // Guardo en una unica lista todos los paths y para cada path aplica map

            let file = File::open(path); // Leo el archivo

            let reader = BufReader::new(file.unwrap()); // Cargo todo el archivo en un buffer
            
            reader.lines()  // Devuelvo el buffer con el archivo
       
        }).map(|l| { // Para cada buffer aplica la funcion map

            let line = if let Ok(l) = l{ // Leo cada linea del buffer
                l
            } else{
                return Err("invalid line")
            };
            
            let video_data = line.split(',').collect::<Vec<&str>>();  // Separo cada campo


            let canal =  if let Some(i) = video_data.get(3){
                *i
            }else{
                return Err("invalid line")
            };
            

            let visitas =  if let Some(v) = video_data.get(7){
                                    *v
                                }else{
                                    return Err("invalid line")
                                };
                            
            let v =  if let Ok(i) = visitas.parse::<i64>(){
                i
            }else{
                return Err("invalid line")
            };

            let mut visitas_canal = HashMap::new();

            *visitas_canal.entry(canal.to_string()).or_insert(0) += v;

            Ok(visitas_canal)
        }).fold(HashMap::new(), |mut acumulador, video_data| {
            if let Ok(vd) = video_data{
                vd.iter().for_each(|(k, v)| *acumulador.entry(k.clone()).or_insert(0) += v);
            }
            
            acumulador
        });

        result
}

// Calcula la cantidad total de vistas por canal (sumando todos los archivos .csv) de manera concurrente
fn vistas_por_canal_par() -> HashMap<String, i64>{
    
    let result = read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/data")).unwrap() // Veo los archivos dentro del directorio 
    .map(|d| d.unwrap().path())
    .collect::<Vec<PathBuf>>()
    .par_iter()  // Cada archvio obtengo su path
        .flat_map(|path| { // Guardo en una unica lista todos los paths y para cada path aplica map

            let file = File::open(path); // Leo el archivo
            let reader = BufReader::new(file.unwrap()); // Cargo todo el archivo en un buffer
            reader.lines().par_bridge()  // Devuelvo el buffer con el archivo
   
    }).map(|l| { // Para cada buffer aplica la funcion map

        let line = if let Ok(l) = l{ // Leo cada linea del buffer
            l
        } else{
            return Err("invalid line")
        };
        
        let video_data = line.split(',').collect::<Vec<&str>>();  // Separo cada campo


        let canal =  if let Some(i) = video_data.get(3){
            *i
        }else{
            return Err("invalid line")
        };
        

        let visitas =  if let Some(v) = video_data.get(7){
                                *v
                            }else{
                                return Err("invalid line")
                            };
                        
        let v =  if let Ok(i) = visitas.parse::<i64>(){
            i
        }else{
            return Err("invalid line")
        };

        let mut visitas_canal = HashMap::new();

        *visitas_canal.entry(canal.to_string()).or_insert(0) += v;

        Ok(visitas_canal)
    }).reduce(|| Ok(HashMap::new()), |mut acumulador, video_data| {
        if let Ok(vd) = video_data{
            vd.iter().for_each(|(k, v)| *acumulador.as_mut().unwrap().entry(k.clone()).or_insert(0) += v);
        }
        
        acumulador
    });

    result.unwrap()
}

fn main() {

    let start = Instant::now();
    let result = vistas_por_canal();
    println!("El tiempo total de manera secuencial es {:?}", start.elapsed());
    
    // //println!("{:?}", result);
    println!("La cantidad total de canales es {}", result.keys().len());

    let start = Instant::now();
    let result_par = vistas_por_canal_par();
    println!("El tiempo total concurrentemente es {:?}", start.elapsed());

    //println!("{:?}", result_par);
    println!("La cantidad total de canales es {}", result_par.keys().len());

}