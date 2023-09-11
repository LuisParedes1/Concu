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
use crate::api_response::{APIResponse};
use print_api_response::print_tracks;

mod api_response;
mod print_api_response;


// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {

    let client = reqwest::Client::new();

    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = "Bizzarap"
    );

    let auth_token = "BQD4xBpOjzUTNKvby2_lfH2b--ZKJ3Zk9DhVUTVdTlJOgwwTNHQ_lTSqh0ulE7DkxH7NKAm5nPL2a6aBN_OB7om5LJfpvJFxPI4OS6FtSnUcJd3KHw4";

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
                        Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                        Err(_) => println!("Hm, the response didn't match the shape we expected."),
                    };
                }
                reqwest::StatusCode::UNAUTHORIZED => {
                    println!("Unauthorized Request");
                }
                _ => {
                    println!("Bad Request");
                }
            }

        },
        Err(_) => println!("Error"),
    }
}
