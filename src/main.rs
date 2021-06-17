use clap::{crate_version, App, Arg, ArgMatches};
use reqwest::Method;
use tokio::time::Instant;

mod pinger;
mod proxy_reader;
mod test_results;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("pingman")
        .version(crate_version!())
        .author("github.com/steele123")
        .about("A cli for http pinging with support for proxies at very fast speeds in concurrency.")
        .subcommand(
            App::new("ping")
                .about("pings a site through http")
                .arg(
                    Arg::new("site")
                        .short('s')
                        .required(true)
                        .about("the site that should get pinged")
                        .value_name("URL")
                        .takes_value(true)
                )
                .arg(
                    Arg::new("pings")
                        .short('p')
                        .default_value("10")
                        .about("the amount of requests that should be sent per site")
                        .value_name("PINGS")
                        .takes_value(true)
                )
                .arg(
                    Arg::new("get")
                        .short('g')
                        .about("whether pingman should send a get request or a head request, head requests are faster")
                )
        )
        .subcommand(
            App::new("proxy")
                .about("pings a site with a proxy")
                .arg(
                    Arg::new("site")
                        .short('s')
                        .about("the site that should get pinged")
                        .long("site")
                        .default_value("https://google.com")
                        .value_name("URL")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("file")
                        .short('f')
                        .about("file with proxies in them")
                        .value_name("PATH")
                        .takes_value(true),
                )
                .arg(
                    // TODO
                    Arg::new("format")
                        .about("the format to parse the proxies from the file")
                        .value_name("FORMAT")
                        .takes_value(true),
                )
                .arg(
                    // TODO
                    Arg::new("proxy")
                        .about("a proxy formatted as ip:port or ip:port:username:password")
                        .value_name("PROXY")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .about("the path to a location on your computer to save the proxies results to in json format")
                        .value_name("PATH")
                        .takes_value(true),
                )
                .arg(
                    // TODO
                    Arg::new("get")
                        .short('g')
                        .about("this will make the requests get requests instead of head requests, this should be slower but not all sites support head requests")
                )
                .arg(
                    Arg::new("pings")
                        .short('p')
                        .default_value("10")
                        .about("amount of requests that each proxy should do")
                        .value_name("Pings")
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("proxy", proxy_matches)) => {
            let site = proxy_matches
                .value_of("site")
                .unwrap_or("https://google.com");
            let pings: i32 = proxy_matches.value_of("pings").unwrap_or("10").parse()?;

            let pinger = pinger::Pinger::new(pings, site);

            if let Some(path) = proxy_matches.value_of("file") {
                let reader = proxy_reader::ProxyReader::new();

                let proxies = reader.read(path).await?;

                let results = pinger.ping_vec(proxies).await?;

                let lock = &results.lock().await;

                let succeeded = &lock.succeeded;

                let failed = &lock.failed;

                println!("Ping Results for {}", &site);

                println!("Proxies Succeeded - {}", succeeded.len());

                println!("Proxies Failed - {}", failed.len());

                for result in succeeded {
                    println!("{} - {} ms", result.ip, result.ping);
                }

                if let Some(path) = proxy_matches.value_of("output") {
                    let deref_lock = &**lock;

                    let json = serde_json::to_string_pretty(&deref_lock)?;

                    tokio::fs::write(path, json).await?;
                }
            }
        }
        Some(("ping", ping_matches)) => {
            let site = ping_matches.value_of("site").unwrap();
            let pings: i32 = ping_matches.value_of("pings").unwrap().parse()?;

            let client = reqwest::Client::new();

            let mut sum: i32 = 0;
            let mut highest: i32 = 0;
            let mut lowest: i32 = 0;

            let mut method = Method::HEAD;

            if ping_matches.is_present("get") {
                method = Method::GET;
            }

            println!("Sending {} pings to {}\n", pings, site);

            for i in 0..=pings {
                let client = client.clone();

                let method = method.clone();

                if i == 0 {
                    client.head(site).send().await.unwrap();
                    continue;
                }

                let instant = Instant::now();

                let resp = client.request(method, site).send().await.unwrap();

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
                site,
                lowest,
                highest,
                sum / pings
            )
        }
        _ => {}
    }

    Ok(())
}
