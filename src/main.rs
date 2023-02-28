mod utils;
mod services;

use clap::{Args, Parser, Subcommand};
use crate::services::weather_service::weather_forecast;

#[derive(Parser, Debug)]
#[command(author = "Oleksii Donskoi")]
#[command(version = "0.1")]
#[command(about = "Does weather forecast", long_about = None)]
struct Cli {
    #[command(subcommand)]
    com: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Configure app to use selected service. Please use: openweather, weatherapi, accuweather or aerisweather")
    Configure { provider: Option<String> },

    /// Get weather by given address 'your address' or with date (dd-mm): get 'your address' date=21-02
    Get(GetArgs),
}

#[derive(Args, Debug)]
struct GetArgs {
    address: Option<String>,

    #[arg(default_value = "date=now")]
    options: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.com {
        SubCommand::Configure { provider } => utils::config::set_provider(provider),
        SubCommand::Get(GetArgs { address, options }) => match address {
            Some(address) => weather_forecast(address, options),
            None => println!("Please set your location as 'city country code': get 'Kyiv UA'"),
        },
    };
}
