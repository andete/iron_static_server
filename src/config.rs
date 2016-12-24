#[derive(Deserialize, Debug)]
pub struct Config {
    pub listen: String,
    pub tls: Option<Tls>,
    pub vhost: Vec<VHost>,
}

#[derive(Deserialize, Debug)]
pub struct VHost {
    pub hostname: String,
    pub static_files: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Tls {
    pub cert: String,
    pub key: String,
}
