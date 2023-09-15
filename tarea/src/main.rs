/*  Following this tutorial https://blog.logrocket.com/making-http-requests-rust-reqwest/


    Authentication Token Expires every hour

    Generate a new one

    curl -X POST "https://accounts.spotify.com/api/token" \
     -H "Content-Type: application/x-www-form-urlencoded" \
     -d "grant_type=client_credentials&client_id=your-client-id&client_secret=your-client-secret"

    Where your-client-id and your-client-secret are generated with the App (https://developer.spotify.com/dashboard)

    https://developer.spotify.com/documentation/web-api/tutorials/getting-started#request-an-access-token

    * API https://developer.spotify.com/documentation/web-api/reference/search
    * Spotify Dev https://developer.spotify.com/

*/


use reqwest::{self, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}};
use tokio::{task};
use crate::api_response::{APIResponse};
use print_api_response::print_tracks;

mod api_response;
mod print_api_response;
mod auth;


/*
    Obtengo las mejores canciones de un artista

    Le pego a la API de Spotify y obtengo un JSON con la info de las canciones
*/
async fn get_songs_by(query_artist:String, auth_token:&str) -> Result<APIResponse, String> {

    let client = reqwest::Client::new();

    let url = format!(
        "https://api.spotify.com/v1/search?q={query_artist}&type=track,artist"
    );

    // chaining .await will yield our query result
    let result = client
                        .get(url)
                        .header(AUTHORIZATION, format!("Bearer {}", auth_token) )
                        .header(CONTENT_TYPE, "application/json")
                        .header(ACCEPT, "application/json")
                        .send()
                        .await;

    match result {
        Ok(response) =>{
            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<APIResponse>().await {
                        Ok(parsed) => return Ok(parsed),
                        Err(_) => return Err("Error Parsing".to_string()),
                    };
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    return Err("Unauthorized Request. Check credentials".to_string());
                }
                _ => {
                    return Err("Bad Request. Check url query".to_string());
                }
            }

        },
        Err(_) => return Err("No response".to_string()),
    }
}

/*
    Obtengo las mejores canciones de una lista de artistas
*/
async fn many_artist_info(artists:Vec<String>, auth_token: String) -> Vec<Result<APIResponse, String>>{

    let mut task_handles = vec![];

    // Creo un task por cada artista
    for artist in artists{
        let auth_token_clone = auth_token.clone(); 
        task_handles.push(task::spawn(async move {
            get_songs_by(artist.to_string(), &auth_token_clone).await
        }));
    }

    let mut responses = vec![];

    // Espero a que terminen todos los tasks
    for handle in task_handles {
        responses.push(
                        match  handle.await{
                            Ok(response) => response,
                            Err(_) => Err("Error".to_string()),
                        }
                    );
    }

    responses
    

} 


fn main(){

    let artists = vec!["Bizzarap".to_string(), "Daft Punk".to_string(), "Maria Becerra".to_string(), "Tini".to_string()];

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Lo hago bloqueante porque sin las credenciales no puedo hacer nada
    let auth_token = match rt.block_on(auth::get_auth_token()) {
        Ok(token) => token,
        Err(mss) => String::from("Error"),
    };

    let responses = rt.block_on(many_artist_info(artists, auth_token));

    for response in responses {

        match response {
            Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
            Err(mss) => println!("{}",mss),
        };
    }

}