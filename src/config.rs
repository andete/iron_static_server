#[derive(Deserialize, Debug)]
pub struct Config {
    tls: Option<Tls>,
    vhost: Vec<VHost>,
}

#[derive(Deserialize, Debug)]
pub struct VHost {
    name: String,
    static_files: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Tls {
    cert: String,
    key: String,
}
