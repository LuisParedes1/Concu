use async_std::task;
use async_std::io::prelude::*;
use async_std::net;

/*  Aca realizo el request.
 Las operaciones con await son las bloqueantes.

 block_on implementa el modelo "piñata" y al finalizar
 la funcion devuelve el Result<String,Err> como si fuese
 una funcion sincronica normal
*/
async fn cheapo_request(host: &str, port: u16, path: &str)
-> std::io::Result<String>
{
    let mut socket = net::TcpStream::connect((host,port)).await?;
    
    let request = format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n",path, host);
    
    socket.write_all(request.as_bytes()).await?;
    socket.shutdown(net::Shutdown::Write)?;
    
    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    
    Ok(response)
}

pub async fn many_requests(requests: Vec<(String, u16, String)>)
-> Vec<std::io::Result<String>>
{
    
    let mut handles = vec![];

    // Creo una tarea por cada request y lo meto en el task pool. 
    // spawn_local guarda futures que luego block_on intercala
    for (host, port, path) in requests {
        handles.push(task::spawn_local(cheapo_owning_request(host,port, path)));
    }
    
    let mut results = vec![];
    
    // Espero que termine todas las tareas
    for handle in handles {
        results.push(handle.await);
    }

    results
}


/*
    This function takes Strings instead of &str references, so
    its future owns the host and path strings itself, and its
    lifetime is 'static
*/
async fn cheapo_owning_request(host: String, port: u16, path:
    String)
    -> std::io::Result<String> {
        cheapo_request(&host, port, &path).await
    }


fn main() {
    let requests = vec![
            ("example.com".to_string(),80, "/".to_string()),
            ("www.red-bean.com".to_string(), 80, "/".to_string()),
            ("en.wikipedia.org".to_string(), 80, "/".to_string()),
        ];
    
    // Block_on es una operacion bloqueante pero intercala entre las 3 tareas (los 3 requests)
    let results = async_std::task::block_on(many_requests(requests));
    
    for result in results {
        match result {
            Ok(response) => println!("{}", response),
            Err(err) => eprintln!("error: {}", err),
        }
    }
}
