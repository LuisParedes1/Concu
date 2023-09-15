use dotenv::dotenv;
use reqwest;

// From this blog https://nunomaduro.com/load_environment_variables_from_dotenv_files_in_your_rust_program
// .env file must be at root directory


/* Obtengo auth token a partir de las credenciales */
pub async fn get_auth_token()-> String{

    dotenv().ok(); // Load env variables from .env file

    let client_id = std::env::var("ClientID").expect("ClientID must be set.");
    let cliente_secret = std::env::var("ClientSecret").expect("ClientSecret must be set.");
    
    let client = reqwest::Client::new();

    let var = client
                            .put("https://accounts.spotify.com/api/token")
                            .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                            .body(format!("grant_type=client_credentials&client_id={}&client_secret={}",
                                    client_id, cliente_secret)
                                )
                            .send()
                            .await;

    "".to_string()
            
}

// "grant_type=client_credentials&client_id=your-client-id&client_secret=your-client-secret"

#[cfg(test)]

mod auth {

    use dotenv::dotenv;
    use super::get_auth_token;

    #[test]
    fn get_auth_returns_something() {
        let result = super::get_auth_token();
        assert!(!result.is_empty());
    }

    #[test]
    fn accesing_cliente_id_and_cliente_secret() {
        dotenv().ok();

        let client_id = std::env::var("ClientID").expect("ClientID must be set.");
        let cliente_secret = std::env::var("ClientSecret").expect("ClientSecret must be set.");

        print!("{} \n {}", client_id, cliente_secret);

        
        assert!(!client_id.is_empty());
        assert!(!cliente_secret.is_empty());
    }

    #[test]
    fn auth_acceses_generates_token_id() {  
        assert!(get_auth_token().is_empty());
    }
}