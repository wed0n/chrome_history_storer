mod chrome_history;
use std::{
    env,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use chrome_history::{ChromeInfo, TEMPORARY_DATABASE_FILE_NAME};
use clap::{Parser, Subcommand};
use log::{error, info};
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Store {
        #[arg(value_name = "chrome_history.json", help = "history json file")]
        path: PathBuf,
    },
    Search {
        #[arg(value_name = "pattern", help = "search regex pattern")]
        pattern: String,
        #[arg(value_name = "chrmoe_history.db3", help = "history dababase file")]
        path: PathBuf,
    },
}
fn main() {
    let mut builder = env_logger::builder();
    builder.format_timestamp_millis();
    if let Err(_) = env::var("RUST_LOG") {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.try_init().unwrap();

    let config = Cli::parse();
    info!("start");
    match config.command {
        Commands::Store { path } => {
            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            match serde_json::from_reader::<_, ChromeInfo>(reader) {
                Ok(result) => {
                    let time_str = result.time_range;
                    fs::rename(
                        TEMPORARY_DATABASE_FILE_NAME,
                        format!("chrome_history_{}.db3", &time_str),
                    )
                    .unwrap();
                }
                Err(_) => {
                    error!("deserialize {} failed", path.to_str().unwrap_or_default())
                }
            };
        }
        Commands::Search { pattern, path } => {
            match Regex::new(&pattern) {
                Ok(pattern) => {
                    let mut db = chrome_history::new_db(path);
                    if let Some(time_str) = chrome_history::search(&mut db, pattern) {
                        fs::rename(
                            TEMPORARY_DATABASE_FILE_NAME,
                            format!("chrome_history_searched_{}.db3", &time_str),
                        )
                        .unwrap();
                    }
                }
                Err(err) => error!("bad regex pattern: {}", err),
            };
        }
    }
    info!("finish");
}
