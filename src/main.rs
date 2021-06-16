use crate::config::{Config, Proxy};
use crate::test_results::TestResults;
use anyhow::{anyhow, Result};
use futures::future::join_all;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration, Instant};

mod config;
mod test_results;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config: Config = if let Ok(config) = config::Config::load().await {
        config
    } else {
        println!("You did not provide a config file, a example config file has been provided to you please rename it to Config.toml\n when you have made the changes you wanted to it.");
        sleep(Duration::from_secs(10)).await;
        return Err(anyhow::anyhow!("No config found."));
    };

    if std::path::Path::new("./proxies.txt").exists() {
        println!("Loading proxies from proxies.txt");

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

            config.proxies.push(proxy);
        }
    }

    println!("Loaded {} proxies from config!", config.proxies.len());

    match reqwest::Client::new().head(&config.site).send().await {
        Ok(_) => {}
        Err(e) => {
            if e.is_connect() {
                let message = format!("Couldn't connect to {}", &config.site);

                println!("{}", message);

                sleep(Duration::from_secs(5)).await;

                return Err(anyhow!(message));
            }
        }
    };

    let results = Arc::new(Mutex::new(TestResults::new()));

    {
        // We add one because the first request initializes most of the request stuff
        let pings = config.pings.unwrap_or(10) + 1;

        let mut handles = Vec::new();

        let site = config.site.clone();

        for proxy in config.proxies {
            let url = format!("{}:{}", proxy.ip, proxy.port);

            let url_clone = url.clone();

            let results_clone = results.clone();

            let site = site.clone();

            let handle = tokio::task::spawn(async move {
                let req_proxy = reqwest::Proxy::all(&url_clone)
                    .unwrap()
                    .basic_auth(&proxy.username.unwrap(), &proxy.password.unwrap());

                let client = reqwest::Client::builder().proxy(req_proxy).build().unwrap();

                let mut sum: i32 = 0;

                for i in 1..=pings {
                    let instant = Instant::now();

                    match client.head(site.clone()).send().await {
                        Ok(_) => {
                            if i != 1 {
                                sum += instant.elapsed().as_millis() as i32;
                            }
                        }
                        Err(e) => {
                            if e.is_connect() {
                                // proxy invalid or site not up
                                println!("Proxy failed to connect...");
                                break;
                            }
                        }
                    };
                }

                if sum != 0 {
                    results_clone.lock().await.add_success(&url, sum / pings);
                } else {
                    results_clone.lock().await.add_failure(&url);
                }
            });

            handles.push(handle);
        }

        join_all(handles).await;
    }

    for result in &results.lock().await.succeeded {
        println!("{} - {} ms", result.ip, result.ping);
    }

    println!(
        "Results for {} proxies",
        results.lock().await.succeeded.len()
    );

    Ok(())
}
