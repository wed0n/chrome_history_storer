use std::i64;

use log::{error, info};
use sqlite::Connection;

use super::HistoryItem;
const SQL_DDL: &str="CREATE TABLE IF NOT EXISTS history (time INTEGER PRIMARY KEY DESC, title TEXT, url TEXT) WITHOUT ROWID;";
pub struct DB {
    con: Connection,
    is_in_transaction: bool,
}

pub fn new_db<T: AsRef<std::path::Path>>(path: T) -> DB {
    let con = sqlite::open(path).unwrap();
    con.execute(SQL_DDL).unwrap();
    return DB {
        con: con,
        is_in_transaction: false,
    };
}

impl DB {
    pub fn get_transaction_status(&self) -> bool {
        return self.is_in_transaction;
    }

    pub fn begin_transaction(&mut self) {
        self.con.execute("BEGIN TRANSACTION;").unwrap();
        self.is_in_transaction = true;
    }

    pub fn end_transaction(&mut self) {
        self.con.execute("END TRANSACTION;").unwrap();
        self.is_in_transaction = false;
    }

    pub fn add(&mut self, item: HistoryItem) {
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

    pub fn select_item(&self) -> impl FnMut() -> Option<HistoryItem> + '_ {
        let mut statement = self
            .con
            .prepare("select * from history order by time desc")
            .unwrap();
        return move || match statement.next() {
            Ok(state) => match state {
                sqlite::State::Row => {
                    let item = HistoryItem {
                        title: statement.read::<String, _>("title").unwrap(),
                        url: statement.read::<String, _>("url").unwrap(),
                        time_usec: statement.read::<i64, _>("time").unwrap(),
                    };
                    Some(item)
                }
                sqlite::State::Done => None,
            },
            Err(err) => {
                error!("select item failed: {}", err.message.unwrap_or_default());
                return None;
            }
        };
    }

    pub fn vacuum(&self) {
        info!("begin VACUUM");
        if let Err(e) = self.con.execute("VACUUM;") {
            error!("VACUUM failed: {}", e.message.unwrap_or_default());
        }
        info!("end VACUUM");
    }
}
