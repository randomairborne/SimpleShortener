use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub database: String,
    pub users: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Edit {
    pub port: u16,
    pub database: String,
    pub users: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Add {
    pub link: String,
    pub destination: String,
    pub username: String,
    pub password: String,
}