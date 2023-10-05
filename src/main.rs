#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate anyhow;
use clap::Parser;

mod config;
mod integration;
mod pulsar;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    config: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let conf = config::Configuration::load(&cli.config).unwrap();

    integration::setup_log(&conf.integration).unwrap();
    integration::register(Box::new(
        pulsar::Integration::new(conf.pulsar.clone()).await.unwrap(),
    ))
    .await;

    integration::start(conf.integration).await.unwrap();
}
