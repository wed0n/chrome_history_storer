use log::error;
use serde::de::{SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt;
use std::marker::PhantomData;

use super::db::new_db;
use super::{batch_insert_item, HistoryItem, TEMPORARY_DATABASE_FILE_NAME};

type ReturnType = String;
struct HistoryVisitor(PhantomData<fn() -> HistoryItem>);
impl<'de> Visitor<'de> for HistoryVisitor {
    type Value = ReturnType;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a nonempty sequence of numbers")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        let mut db = new_db(TEMPORARY_DATABASE_FILE_NAME);
        return batch_insert_item::<S::Error>(&mut db, || {
            seq.next_element::<HistoryItem>().map_err(|err| {
                error!("deserialize failed: {}", err.to_string());
                err
            })
        });
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ReturnType, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = HistoryVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}
