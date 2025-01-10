mod db;
mod deserialize;
use deserialize::deserialize;
use serde::{Deserialize, Serialize};

pub const TEMPORARY_DATABASE_FILE_NAME:&str="chrome_history_tmp.db3";
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
