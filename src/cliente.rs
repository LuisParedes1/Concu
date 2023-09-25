/*
    Este cliente es sincronico. Es decir, no devuelve futures
*/

// send() devuelve el Future

use reqwest::{self}; // https://docs.rs/reqwest/latest/reqwest/
use std::{collections::HashMap, fmt::Error};

#[derive(Clone)]
pub struct Cliente {
    conection: reqwest::Client,
}

impl Cliente {
    // Devuelve un cliente con una conexion permanente establecida
    pub fn new() -> Self {
        Self {
            conection: reqwest::Client::new(),
        }
    }

    // GET command
    pub async fn get(self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        self.conection.get(url).send().await
    }

    // PUT command
    // form_data contains header and body info
    pub async fn post(
        self,
        url: String,
        form_data: HashMap<&str, &str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        self.conection
            .post(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .form(&form_data)
            .send()
            .await
    }

    // DELETE command
    pub async fn delete(self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        self.conection.delete(url).send().await
    }
}
