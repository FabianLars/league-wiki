#![forbid(unsafe_code)]

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use clap::{ArgEnum, Clap};
use league_wiki::{champs, discounts, positions, rotation, set};
use mw_tools::Client;

#[derive(ArgEnum, Debug, PartialEq)]
enum LeagueType {
    Champs,
    Champions,
    Discount,
    Discounts,
    Positions,
    Rotation,
    Rotations,
    Set,
}

#[derive(Clap, Debug, PartialEq)]
struct Cli {
    #[clap(arg_enum)]
    command: LeagueType,

    #[clap(short, long, env = "FANDOM_BOT_NAME", hide_env_values = true)]
    name: String,
    #[clap(short, long, env = "FANDOM_BOT_PASSWORD", hide_env_values = true)]
    password: String,
    #[clap(
        short,
        long,
        default_value = "https://leagueoflegends.fandom.com/de/api.php"
    )]
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let cli = Cli::parse();
    let mut client = Client::new()?
        .with_url(&cli.url)
        .with_credentials(&cli.name, &cli.password);
    client.login().await?;
    let client = client;

    match cli.command {
        LeagueType::Champs | LeagueType::Champions => champs().await,
        LeagueType::Discount | LeagueType::Discounts => {
            discounts(&client, get_client_path()?).await
        }
        LeagueType::Positions => positions(&client).await,
        LeagueType::Rotation | LeagueType::Rotations => rotation(&client).await,
        LeagueType::Set => set(&client).await,
    }?;

    Ok(())
}

fn get_client_path() -> Result<PathBuf> {
    use sysinfo::{ProcessExt, RefreshKind, System, SystemExt};

    let system = System::new_with_specifics(RefreshKind::new().with_processes());

    let process = system.process_by_name("LeagueClient.exe");

    if let Some(p) = process.get(0) {
        if let Some(path) = p.exe().parent() {
            let mut path = path.to_path_buf();
            path.push("lockfile");
            return Ok(path);
        }
    }

    Err(anyhow!("Can't find lockfile. Make sure that the League Client is running. If it still doesn't work, try specifying the path to the lockfile yourself."))
}
