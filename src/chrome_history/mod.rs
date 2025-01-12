mod db;
mod deserialize;
mod search;
use chrono::DateTime;
pub use db::{new_db, DB};
use deserialize::deserialize;
use log::info;
pub use search::search;
use serde::{Deserialize, Serialize};

pub const TEMPORARY_DATABASE_FILE_NAME: &str = "chrome_history_tmp.db3";
const BATCH: i64 = 0x4000;
#[derive(Deserialize)]
pub struct ChromeInfo {
    #[serde(rename = "Browser History", deserialize_with = "deserialize")]
    pub time_range: String,
}

#[derive(Serialize, Deserialize)]
pub struct HistoryItem {
    pub title: String,
    pub url: String,
    pub time_usec: i64,
}

pub fn format_time(begin_time: i64, end_time: i64) -> String {
    let format_str = "%Y%m%d%H%M%S";
    let begin_time = DateTime::from_timestamp_micros(begin_time)
        .unwrap()
        .format(format_str)
        .to_string();
    let end_time = DateTime::from_timestamp_micros(end_time)
        .unwrap()
        .format(format_str)
        .to_string();
    let time_str = format!("{}~{}", begin_time, end_time);
    return time_str;
}

pub fn batch_insert_item<T>(
    db: &mut DB,
    mut get_item: impl FnMut() -> Result<Option<HistoryItem>, T>,
) -> Result<String, T> {
    let end_time;
    let mut begin_time = 0i64;
    let mut batch_count = 0i64;
    let mut count = 0i64;
    match get_item()? {
        Some(item) => {
            end_time = item.time_usec;
            db.begin_transaction();
            db.add(item);
        }
        None => return Ok(String::default()),
    }
    loop {
        match get_item()? {
            Some(item) => {
                begin_time = item.time_usec;
                db.add(item);
                count += 1;
                if count >= BATCH {
                    batch_count += 1;
                    db.end_transaction();
                    info!("batch count {}", batch_count);
                    db.begin_transaction();
                    count = 0;
                }
            }
            None => break,
        }
    }
    if db.get_transaction_status() {
        db.end_transaction();
    }
    db.vacuum();

    return Ok(format_time(begin_time, end_time));
}
