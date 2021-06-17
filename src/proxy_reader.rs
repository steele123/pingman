use crate::pinger::Proxy;
use anyhow::Result;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

pub struct ProxyReader;

impl ProxyReader {
    pub fn new() -> Self {
        Self
    }

    pub async fn read(&self, path: &str) -> Result<Vec<Proxy>> {
        return if std::path::Path::new(path).exists() {
            let mut proxies = Vec::new();

            let file = File::open("./proxies.txt").await?;
            let reader = BufReader::new(file);

            let mut lines = reader.lines();

            loop {
                let line = lines.next_line().await?;

                if line.is_none() {
                    break;
                }

                let line = line.unwrap();

                let split = line.split(':');
                let data: Vec<&str> = split.collect();

                if data.is_empty() {
                    continue;
                }

                let proxy = Proxy {
                    ip: data[0].to_string(),
                    port: data[1].parse()?,
                    username: Some(data[2].to_string()),
                    password: Some(data[3].to_string()),
                };

                proxies.push(proxy);
            }

            Ok(proxies)
        } else {
            Err(anyhow::anyhow!("Couldn't find the path to the proxies"))
        };
    }
}
