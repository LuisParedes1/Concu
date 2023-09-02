use std::thread::spawn;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::io::*;


fn process_files(filenames: Vec<String>) -> Result<()> {
    for document in filenames {
        let text = load(&document)?; 
        let results = process(text); 
        save(&document, results)?;
    }
    Ok(())
}

fn main() {

    /* Es equivalente a estas 3 lineas

        read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/data")).unwrap()
        .map(|d| d.unwrap().path())
        .collect::<Vec<PathBuf>>()
    */

    let file = if let Ok(dir) = read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/data")){
        dir
    }else{
        return;
    };
    
    let mut paths = vec![];

    for d in file{
        if let Ok(dir_entry) = d{
            paths.push(dir_entry.path())
        }
    }


    /* Es equivalente a esto

        .par_iter()
        .flat_map(|path| {
            let file = File::open(path);
            let reader = BufReader::new(file.unwrap());
            reader.lines().par_bridge()
        })
        .map(|l| {
            let line = l.unwrap();
            let words = line.split(' ');
            thread::sleep(Duration::from_millis(100));
            let mut counts = HashMap::new();
            words.for_each(|w| *counts.entry(w.to_string()).or_insert(0) += 1);
            counts
        })
        .reduce(|| HashMap::new(), |mut acc, words| {
            words.iter().for_each(|(k, v)| *acc.entry(k.clone()).or_insert(0) += v);
            acc
        });
    
    */


    
    let mut thread_handlers = vec![];

    // Para cada archivo
    for p in paths{
        

        thread_handlers.push(spawn(move || {
            
            let file = match File::open(p){
                Ok(file) => file,
                Err(_) => return
            };
    
            let reader = BufReader::new(file);
            let lines = reader.lines();


            //let words = lines.split(' ');

            

        }
        ));
    }

    for thread_handler in thread_handlers{
        if let Ok(_) = thread_handler.join(){}
        else{
            // Error Handling
            println!("Algo raro sucedio")
        }
    }

    
    
    // println!("Llegue al final exitosamente!");
}
