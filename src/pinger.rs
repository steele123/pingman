use crate::test_results::TestResults;
use anyhow::{Error, Result};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Instant;

pub struct Pinger {
    pings: i32,
    url: String,
}

#[derive(Debug)]
pub struct Proxy {
    pub ip: String,
    pub port: i32,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Pinger {
    pub fn new(pings: i32, url: &str) -> Self {
        Self {
            pings,
            url: url.to_string(),
        }
    }

    pub async fn ping(&self, proxy: &Proxy) -> Result<i32> {
        Ok(ping(proxy, &self.url, self.pings).await?)
    }

    pub async fn ping_vec(&self, proxies: Vec<Proxy>) -> Result<Arc<Mutex<TestResults>>> {
        let results = Arc::new(Mutex::new(TestResults::new()));

        let mut handles = Vec::new();

        let url = self.url.clone();

        let pings = self.pings.clone();

        for proxy in proxies {
            let results = results.clone();

            let url = url.clone();

            handles.push(tokio::task::spawn(async move {
                let proxy_url = format!("{}:{}", proxy.ip, proxy.port);

                match ping(&proxy, &url, pings).await {
                    Ok(ping) => {
                        results.lock().await.add_success(&proxy_url, ping);
                    }
                    Err(e) => {
                        println!("{}", e);
                        results.lock().await.add_failure(&proxy_url);
                    }
                };
            }));
        }

        futures::future::join_all(handles).await;

        Ok(results)
    }
}

pub async fn ping(proxy: &Proxy, url: &String, pings: i32) -> Result<i32> {
    let req_proxy = reqwest::Proxy::all(format!("{}:{}", proxy.ip, proxy.port))
        .unwrap()
        .basic_auth(
            &proxy.username.as_ref().unwrap(),
            &proxy.password.as_ref().unwrap(),
        );

    let client = reqwest::Client::builder().proxy(req_proxy).build().unwrap();

    let mut sum: i32 = 0;

    for i in 1..=pings {
        let instant = Instant::now();

        match client.head(url).send().await {
            Ok(_) => {
                if i != 1 {
                    let elapsed = instant.elapsed().as_millis() as i32;

                    sum += elapsed;
                }
            }
            Err(e) => {
                if e.is_connect() {
                    // proxy invalid or site not up
                    return Err(anyhow::anyhow!("The proxy seems to be invalid"));
                }
            }
        };
    }

    Ok(sum / pings)
}
