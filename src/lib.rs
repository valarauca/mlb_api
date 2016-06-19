
extern crate hyper;
extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::env;
use std::io;
use std::io::prelude::*;
use std::time::Duration;
use hyper::client::{Client, Response};
use hyper::status::StatusCode;
use hyper::header::Connection;


//What stage the error occured at
#[derive(Debug)]
pub enum RequestError {
    Get(hyper::error::Error),
    Status(StatusCode),
    Read(std::io::Error),
    Json(rustc_serialize::json::BuilderError)
}

//make player profile json request
pub fn player_profile_json(
    client: &mut Client,
    access_level: &str,
    version: &str,
    player_id: &str,
    api_key: &str ) -> Result< Json, RequestError> {

    //make a empty string to hold url
    let mut url = String::with_capacity(4000);
    //construct url
    url.push_str("https://api.sportradar.us/mlb-");
    url.push_str(access_level);
    url.push_str(version);
    url.push_str("/players/");
    url.push_str(player_id);
    url.push_str("/profile.");
    url.push_str("json");
    url.push_str("?api_key=");
    url.push_str(api_key);

    //make request
    let mut response = match client.get(&url).send() {
        Ok(x) => x,
        Err(e) => return Err(RequestError::Get(e))
    };

    //check response code
    match response.status {
        hyper::status::StatusCode::Ok => { },
        x => return Err(RequestError::Status(x))
    };

    //read body
    let mut read_buffer = String::with_capacity(8000);
    match response.read_to_string( &mut read_buffer ) {
        Ok(_) => { },
        Err(e) => return Err(RequestError::Read(e))
    };

    //convert to json (and return)
    match Json::from_str( &read_buffer ) {
        Ok(x) => Ok(x),
        Err(e) => return Err(RequestError::Json(e))
    }
}

//get injury list
pub fn injury_list_json(
    client: &mut Client,
    access_level: &str,
    version: &str,
    api_key: &str ) -> Result< Json, RequestError> {

    //make a empty string to hold url
    let mut url = String::with_capacity(4000);
    //construct url
    url.push_str("https://api.sportradar.us/mlb-");
    url.push_str(access_level);
    url.push_str(version);
    url.push_str("/league/injuries.json?api_key=");
    url.push_str(api_key);

    //make request
    let mut response = match client.get(&url).send() {
        Ok(x) => x,
        Err(e) => return Err(RequestError::Get(e))
    };

    //check response code
    match response.status {
        hyper::status::StatusCode::Ok => { },
        x => return Err(RequestError::Status(x))
    };

    //read body
    let mut read_buffer = String::with_capacity(8000);
    match response.read_to_string( &mut read_buffer ) {
        Ok(_) => { },
        Err(e) => return Err(RequestError::Read(e))
    };

    //convert to json (and return)
    match Json::from_str( &read_buffer ) {
        Ok(x) => Ok(x),
        Err(e) => return Err(RequestError::Json(e))
    }
}
