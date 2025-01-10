use std::i64;

use chrono::DateTime;
use log::{error, info};
use sqlite::Connection;

use super::{HistoryItem, TEMPORARY_DATABASE_FILE_NAME};
const BATCH: i64 = 10000;
const SQL_DDL: &str="CREATE TABLE IF NOT EXISTS history (time INTEGER PRIMARY KEY DESC, title TEXT, url TEXT) WITHOUT ROWID;";
pub struct DB {
    con: Connection,
    is_in_transaction: bool,
    count: i64,
    begin: i64,
    end: i64,
    batch_count: i64,
}

pub fn new_db() -> DB {
    let con = sqlite::open(TEMPORARY_DATABASE_FILE_NAME).unwrap();
    con.execute(SQL_DDL).unwrap();
    return DB {
        con: con,
        is_in_transaction: false,
        count: 0,
        begin: i64::MAX,
        end: 0,
        batch_count: 0,
    };
}

impl DB {
    fn begin_transaction(&mut self) {
        if self.is_in_transaction {
            panic!("bad transaction change");
        }
        self.con.execute("BEGIN TRANSACTION;").unwrap();
        self.is_in_transaction = true;
    }

    fn end_transaction(&mut self) {
        if !self.is_in_transaction {
            panic!("bad transaction change");
        }
        self.con.execute("END TRANSACTION;").unwrap();
        self.is_in_transaction = false;
        self.batch_count += 1;
    }

    pub fn add(&mut self, item: HistoryItem) {
        if !self.is_in_transaction {
            self.begin_transaction();
        }
        let time_usec = item.time_usec;
        if time_usec < self.begin {
            self.begin = time_usec;
        }
        if time_usec > self.end {
            self.end = time_usec;
        }

        {
            let mut state = self
                .con
                .prepare("INSERT INTO history(time, title, url) VALUES(?, ?, ?);")
                .unwrap();
            state.bind((1, item.time_usec)).unwrap();
            state.bind((2, item.title.as_str())).unwrap();
            state.bind((3, item.url.as_str())).unwrap();
            if let Err(_) = state.next() {
                error!(
                    "insert faliled: {}",
                    serde_json::to_string(&item).unwrap_or("get error message failed".to_string())
                );
            }
        }

        self.count += 1;
        if self.count >= BATCH {
            self.end_transaction();
            info!("batch count {}", self.batch_count);
            self.begin_transaction();
            self.count = 0;
        }
    }

    pub fn export_time(&self) -> String {
        let format_str = "%Y%m%d%H%M%S";
        let begin_time = DateTime::from_timestamp_micros(self.begin)
            .unwrap()
            .format(format_str)
            .to_string();
        let end_time = DateTime::from_timestamp_micros(self.end)
            .unwrap()
            .format(format_str)
            .to_string();
        let time_str = format!("{}~{}", begin_time, end_time);
        return time_str;
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        if self.is_in_transaction {
            self.end_transaction();
        }
        info!("begin VACUUM");
        if let Err(e) = self.con.execute("VACUUM;") {
            error!("VACUUM failed: {}", e.message.unwrap_or("".to_string()));
        }
        info!("end VACUUM");
    }
}
