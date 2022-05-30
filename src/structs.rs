use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Add {
    pub link: String,
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Edit {
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Qr {
    pub destination: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddUser {
    pub username: String,
    pub password: String,
}
