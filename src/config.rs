use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub listen: HashMap<String, Listen>,
    pub vhost: HashMap<String, VHost>,
}

#[derive(Deserialize, Debug)]
pub struct Listen {
    pub address:String,
    pub tls: Option<Tls>,
}

#[derive(Deserialize, Debug)]
pub struct VHost {
    pub listen: String,
    pub hostname: String,
    pub static_files: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Tls {
    pub cert:String,
    pub key:String,
}
