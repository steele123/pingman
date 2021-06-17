use clap::{crate_version, App, Arg, ArgMatches};

mod pinger;
mod proxy_reader;
mod test_results;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("pingman")
        .version(crate_version!())
        .author("Steele Scott")
        .about("A cli for pinging and testing proxies at very fast speeds in concurrency.")
        .subcommand(
            App::new("proxy")
                .about("Pings a site with a proxy")
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
                    Arg::new("format")
                        .about("the format to parse the proxies from the file")
                        .value_name("FORMAT")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("proxy")
                        .about("a proxy formatted as ip:port or ip:port:username:password")
                        .value_name("PROXY")
                        .takes_value(true),
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

                println!("Proxies Succeeded - {}", succeeded.len());

                println!("Proxies Failed - {}", failed.len());

                for result in succeeded {
                    println!("{} - {} ms", result.ip, result.ping);
                }
            }
        }
        Some(("ping", ping_matches)) => {}
        _ => {}
    }

    Ok(())
}
