use fs::File;
use serde::Deserialize;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub site: String,
    pub pings: Option<i32>,
    pub proxies: Vec<Proxy>,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub ip: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Config {
    pub async fn load() -> anyhow::Result<Self> {
        if Path::new("Config.toml").exists() {
            let file_contents = fs::read(Path::new("./Config.toml")).await?;

            return Ok(toml::from_slice(&file_contents)?);
        } else if !Path::new("./ExampleConfig.toml").exists() {
            let mut file = File::create("./ExampleConfig.toml").await?;

            let default_toml = toml::toml! {
                pings = 10
                site = "https://steele.gg"

                [[proxies]]
                ip = "127.0.0.1"
                password = "password"
                port = 8080
                username = "username"

                [[proxies]]
                ip = "127.0.0.1"
                password = "password"
                port = 8080
                username = "username"
            };

            file.write_all(default_toml.to_string().as_bytes()).await?;
        }
        Err(anyhow::anyhow!("No config exists!"))
    }
}
