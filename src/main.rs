use crate::cli::{Opt, Subcommand};
use clap::Clap;
use reqwest::Method;
use tokio::time::Instant;

mod cli;
mod pinger;
mod proxy_reader;
mod test_results;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts: Opt = Opt::parse();

    match opts.subcmd {
        Subcommand::Proxy(proxy) => {
            let pinger = pinger::Pinger::new(proxy.pings, &proxy.site);

            if let Some(path) = proxy.file {
                let proxies = proxy_reader::ProxyReader::new().read(&path).await?;

                let results = pinger.ping_vec(proxies).await?;

                let lock = &results.lock().await;

                let succeeded = &lock.succeeded;

                let failed = &lock.failed;

                println!("Ping Results for {}", &proxy.site);

                println!("Proxies Succeeded - {}", succeeded.len());

                println!("Proxies Failed - {}", failed.len());

                for result in succeeded {
                    println!("{} - {} ms", result.ip, result.ping);
                }

                if let Some(path) = proxy.output {
                    let deref_lock = &**lock;

                    let json = serde_json::to_string_pretty(&deref_lock)?;

                    tokio::fs::write(path, json).await?;
                }
            }
        }
        Subcommand::Ping(ping) => {
            let client = reqwest::Client::new();

            let mut sum: i32 = 0;
            let mut highest: i32 = 0;
            let mut lowest: i32 = 0;

            let method = if ping.get { Method::GET } else { Method::HEAD };

            println!("Sending {} pings to {}\n", ping.pings, ping.site);

            for i in 0..=ping.pings {
                let client = client.clone();

                let method = method.clone();

                if i == 0 {
                    client.head(&ping.site).send().await.unwrap();
                    continue;
                }

                let instant = Instant::now();

                let resp = client.request(method, &ping.site).send().await.unwrap();

                let elapsed = instant.elapsed().as_millis() as i32;

                if elapsed < lowest || lowest == 0 {
                    lowest = elapsed;
                } else if elapsed > highest {
                    highest = elapsed;
                }

                println!(
                    "[Ping {}] Received Code {}: bytes={} time={}ms",
                    i,
                    resp.status(),
                    resp.bytes().await.unwrap().len(),
                    elapsed,
                );

                sum += elapsed;
            }

            println!(
                "\nPings stats for {}\nFastest = {} Slowest = {} Average = {}ms",
                ping.site,
                lowest,
                highest,
                sum / ping.pings
            )
        }
        _ => {
            unreachable!()
        }
    }

    Ok(())
}
