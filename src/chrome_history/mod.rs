mod deserialize;
mod db;
use deserialize::deserialize;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ChromeInfo {
    #[serde(rename = "Browser History", deserialize_with = "deserialize")]
    pub time_range: String,
}

#[derive(Serialize,Deserialize)]
pub struct HistoryItem {
   pub title: String,
   pub url: String,
   pub time_usec: i64,
}
