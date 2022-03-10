use crate::cli::{Opt, Subcommand};
use clap::Clap;
use reqwest::Method;
use owo_colors::OwoColorize;
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

                println!(
                    "Sending {} pings per proxy to {}\n",
                    proxy.pings, &proxy.site
                );

                println!("Proxies Succeeded - {}", succeeded.len());

                println!("Proxies Failed - {}", failed.len());

                for result in succeeded {
                    println!(
                        "Results for {}:{} - Average = {}ms",
                        result.ip, result.port, result.average_ping
                    );
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

                let status = resp.status();

                let mut status_text;

                if status.is_success() {
                    status_text = format!("{}", status.as_u16().bright_green())
                } else {
                    status_text = format!("{}", status.as_u16().bright_red())
                }

                println!(
                    "[Ping {}] Code = {} Bytes = {} Time = {}ms",
                    i,
                    status_text,
                    resp.bytes().await.unwrap().len().bright_blue(),
                    elapsed.bright_yellow(),
                );

                sum += elapsed;
            }

            println!(
                "\nPings stats for {}\nFastest = {} Slowest = {} Average = {}ms",
                ping.site.bright_cyan(),
                lowest.bright_green(),
                highest.bright_red(),
                (sum / ping.pings).bright_blue()
            );

            println!("      Pings = {} Method = {}", ping.pings.bright_yellow(), (if ping.get {  "GET" } else { "HEAD" }).bright_magenta())
        }
        _ => {
            unreachable!()
        }
    }

    Ok(())
}
