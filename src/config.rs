#[derive(Deserialize, Debug)]
struct Config {
    tls: Option<Tls>,
    vhost: Vec<VHost>,
}

#[derive(Deserialize, Debug)]
struct VHost {
    name: String,
    root: Option<String>
}

#[derive(Deserialize, Debug)]
struct Tls {
    cert: String,
    key: String,
}
