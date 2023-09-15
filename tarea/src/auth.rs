use std::collections::HashMap;

use dotenv::dotenv;
use reqwest;

// From this blog https://nunomaduro.com/load_environment_variables_from_dotenv_files_in_your_rust_program
// .env file must be at root directory


/* Obtengo auth token a partir de las credenciales */
pub async fn get_auth_token()-> Result<String, String>{

    dotenv().ok(); // Load env variables from .env file

    let spotify_url = "https://accounts.spotify.com/api/token";
    let client_id = std::env::var("ClientID").expect("ClientID must be set.");
    let client_secret = std::env::var("ClientSecret").expect("ClientSecret must be set.");
    let grant_type = "client_credentials";

    let client = reqwest::Client::new();

    let mut form_data = HashMap::new();
    form_data.insert("grant_type", grant_type);
    form_data.insert("client_id", &client_id);
    form_data.insert("client_secret", &client_secret);

    let response = client
                                                .post(spotify_url)
                                                .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                                                .form(&form_data)
                                                .send()
                                                .await;

    match response {
        Ok(response) =>{
            match response.status().is_success() {
                true => {  let response = response.text().await.unwrap();

                            Ok(response.split(":").collect::<Vec<&str>>()[1].split(",").collect::<Vec<&str>>()[0].replace("\"", "") )
                        },
                false => {Err("Error getting auth token\n".to_string())}   
            }
        },
        Err(error) => {
            Err(format!("Error in request, {}", error.to_string()))
        }
    }          
}

#[cfg(test)]

mod auth {

    use dotenv::dotenv;
    use super::get_auth_token;

    #[test]
    fn accesing_cliente_id_and_cliente_secret() {
        dotenv().ok();

        let client_id = std::env::var("ClientID").expect("ClientID must be set.");
        let client_secret = std::env::var("ClientSecret").expect("ClientSecret must be set.");
        
        assert!(!client_id.is_empty());
        assert!(!client_secret.is_empty());
    }

    #[test]
    fn auth_acceses_generates_token_id() {  

        let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

        let response = rt.block_on(get_auth_token());

        assert!(!response.unwrap().contains("Error"));       
    }
}