use clap::{crate_version, AppSettings, Clap};

#[derive(Clap)]
#[clap(version = crate_version!(), author = "github.com/steele123")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opt {
    #[clap(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Clap)]
pub enum Subcommand {
    Ping(Ping),
    Proxy(Proxy),
}

/// various methods of pinging through http
#[derive(Clap)]
pub struct Ping {
    /// the site that should get pinged
    #[clap(short, long, default_value = "https://google.com")]
    pub site: String,
    /// the amount of requests that should be sent per site
    #[clap(short, default_value = "10")]
    pub pings: i32,
    /// whether or not pingman should use get requests instead of head requests, get requests are much slower but could be a more valid test for you
    #[clap(short)]
    pub get: bool,
}

/// various methods of pinging with a proxy / proxies
#[derive(Clap)]
pub struct Proxy {
    /// the site that should get pinged
    #[clap(short, long, default_value = "https://google.com")]
    pub site: String,
    /// file with proxies in them
    #[clap(short)]
    pub file: Option<String>,
    // TODO
    /// the format to parse proxies from the specified file
    pub format: Option<String>,
    // TODO
    /// a proxy formatted by default as ip:port ip:port:username:password
    pub proxy: Option<String>,
    /// the path to a location on your computer to save the test results in json format
    #[clap(short)]
    pub output: Option<String>,
    /// whether or not pingman should use get requests instead of head requests, get requests are much slower but could be a more valid test for you
    #[clap(short)]
    pub get: bool,
    /// amount of requests that each proxy should do
    #[clap(short, default_value = "10")]
    pub pings: i32,
}
