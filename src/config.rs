#[derive(Deserialize, Debug)]
struct Config {
    version: String,
    vhost: Vec<VHost>,
}

#[derive(Deserialize, Debug)]
struct VHost {
    name: String,
    root: Option<String>
}
