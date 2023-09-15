use std::collections::HashMap;

use futures::future::join_all;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Error,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde_derive::Deserialize;
use tokio::runtime;

#[derive(Debug, Deserialize, Clone)]
struct Observation {
    //speciesCode: String,
    comName: String,
    sciName: String,
    //locId: String,
    locName: String,
    //obsDt: String,
    //lat: f64,
    //lng: f64,
    obsValid: bool,
    //obsReviewed: bool,
    //locationPrivate: bool,
    //subId: String,
}

async fn get_responses(urls: Vec<String>) -> Result<Vec<Vec<Observation>>, Error> {
    let mut futures = vec![];

    let client = Client::builder()
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                "x-ebirdapitoken",
                HeaderValue::from_str("fahfdkof0q19").unwrap(),
            );
            headers
        })
        .build()?;

    for url in urls {
        let body = client.get(&url).send().await?.json();
        futures.push(body);
    }
    // Espera a que todas las futuras se completen antes de continuar.
    let responses: Result<Vec<Vec<Observation>>, Error> =
        join_all(futures).await.into_iter().collect();

    // Devuelve el resultado de las respuestas.
    Ok(responses?)
}

fn print_all(observed: &Vec<Vec<Observation>>) {
    let zones = vec![
        "Ciudad de Buenos Aires",
        "Provincia de Buenos Aires",
        "Rio Negro",
    ];

    for (i, response) in observed.iter().enumerate() {
        println!("Area: {}", zones[i]);
        for obs in response {
            println!(
                "observed a {}, better known as {} in: {}",
                obs.sciName, obs.comName, obs.locName
            );
        }
        println!("");
    }
}

fn join_by_location(obs: &Vec<Observation>) -> HashMap<String, Vec<Observation>> {
    let obs_map = obs
        .par_iter()
        .map(|bird| {
            let mut counts: HashMap<String, Vec<Observation>> = HashMap::new();
            counts.insert(bird.locName.clone(), vec![bird.clone()]);
            counts
        })
        .reduce(
            || HashMap::new(),
            |mut acc, words| {
                words.iter().for_each(|(k, v)| {
                    acc.entry(k.clone())
                        .or_insert_with(Vec::new)
                        .extend(v.iter().cloned())
                });
                acc
            },
        );
    obs_map
}

async fn async_main() -> Result<Vec<Vec<Observation>>, Error> {
    let urls = vec![
        "https://api.ebird.org/v2/data/obs/AR-C/recent".to_string(),
        "https://api.ebird.org/v2/data/obs/AR-B/recent".to_string(),
        "https://api.ebird.org/v2/data/obs/AR-R/recent".to_string(),
    ];
    let responses = get_responses(urls).await?;

    Ok(responses)
}

fn main() -> Result<(), Error> {
    let rt = runtime::Runtime::new().unwrap();
    let result = rt.block_on(async_main())?;
    print_all(&result);

    let maps: Vec<HashMap<String, Vec<Observation>>> =
        result.iter().map(|v| join_by_location(v)).collect();

    if let Some(entries) = maps[0].get("Reserva Ecológica Ciudad Universitaria - Costanera Norte")
    {
        println!(
            "Se cargo el hashmap, los pajaros avistados en la Reserva de Ciudad Universitaria:"
        );
        for bird in entries.iter() {
            println!("- {}, mejor conocido como {}", bird.sciName, bird.comName);
        }
    }
    Ok(())

    //"Reserva Ecológica Ciudad Universitaria - Costanera Norte"
}
