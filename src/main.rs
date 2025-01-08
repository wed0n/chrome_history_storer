mod chrome_history;
use std::{
    env,
    fs::{self, File}, io::BufReader,
};

use chrome_history::ChromeInfo;
use log::{error, info};

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .try_init();
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("args error");
        return;
    }
    info!("start dump");
    let path = &args[1];
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let result: ChromeInfo = serde_json::from_reader(reader).unwrap();
    let time_str = result.time_range;
    fs::rename("history.db3", format!("chrome_history_{}.db3", &time_str)).unwrap();
    info!("Finish");
}
