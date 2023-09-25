use reqwest::{self, header::{AUTHORIZATION, CONTENT_TYPE, ACCEPT}}; // https://docs.rs/reqwest/latest/reqwest/
use std::{collections::HashMap, fmt::Error};

struct Client{
    conection:Client,
}

impl Client{

    // Devuelve un cliente con una conexion permanente establecida
    pub fn new() -> Result<Self, Error>{
        
        match reqwest::Client::new(){
            Ok(conec) => {
                return Ok(Self{
                    conection: conec,
                })
            },
            Err(e)=>{
                return Err(e)
            },
        }
    }
   
    // GET command 
    async fn get(url:String){
        self.cliente.get(url)
    }

    // PUT command
    // form_data contains header and body info
    async fn put(url:String, form_data:HashMap){
        self.cliente
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
        .form(&form_data)
        .send()
    }


    // DELETE command
    async fn delete(url:String){
        self.cliente.delete(url)
    }
}