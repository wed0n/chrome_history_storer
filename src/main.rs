mod chrome_history;
use std::{
    env,
    fs::{self, File},
    io::BufReader,
};

use chrome_history::{ChromeInfo, TEMPORARY_DATABASE_FILE_NAME};
use log::{error, info};

fn main() {
    let mut builder = env_logger::builder();
    builder.format_timestamp_millis();
    if let Err(_) = env::var("RUST_LOG") {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.try_init().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("args error");
        return;
    }
    info!("start dump");
    let path = &args[1];
    let file = File::open(path).unwrap();
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
            error!("deserialize {} failed", path)
        }
    };

    info!("Finish");
}
