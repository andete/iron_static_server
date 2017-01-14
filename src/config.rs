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
    pub redirect: Option<Redirect>,
}

#[derive(Deserialize, Debug)]
pub struct Tls {
    pub identity:String,
    pub secret:String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Redirect {
    pub host:String,
    pub scheme:Option<String>,
    pub port:Option<u16>,
}
