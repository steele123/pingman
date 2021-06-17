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

#[derive(Clap)]
pub struct Ping {
    #[clap(short, long, default_value = "https://google.com")]
    pub site: String,
    #[clap(short, default_value = "10")]
    pub pings: i32,
    #[clap(short)]
    pub get: bool,
}

#[derive(Clap)]
pub struct Proxy {
    #[clap(short, long, default_value = "https://google.com")]
    pub site: String,
    #[clap(short)]
    pub file: Option<String>,
    // TODO
    pub format: String,
    // TODO
    pub proxy: String,
    #[clap(short)]
    pub output: Option<String>,
    #[clap(short)]
    pub get: bool,
    #[clap(short, default_value = "10")]
    pub pings: i32,
}
