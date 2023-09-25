use crate::cliente::Cliente;

mod cliente;

fn main() {
    let cliente = Cliente::new();
    let mut url = "https://www.google.com";

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // GET
    let get_response = runtime.block_on(cliente.clone().get(url.to_string()));

    println!("GET status code: {}", get_response.unwrap().status());

    // POST

    // form_data contiene el header y body de POST (Ver https://docs.rs/reqwest/latest/reqwest/#forms)
    let mut form_data = std::collections::HashMap::new();
    form_data.insert("foo", "bar");
    form_data.insert("baz", "quux");

    let post_response = runtime.block_on(
        cliente
            .clone()
            .post("http://httpbin.org/post".to_string(), form_data),
    );
    println!("POST status code: {}", post_response.unwrap().status());

    // DELETE
    let delete_response = runtime.block_on(cliente.clone().delete(url.to_string()));
    println!("DELETE status code: {}", delete_response.unwrap().status());
}
