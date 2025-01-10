use log::error;
use serde::de::{SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt;
use std::marker::PhantomData;

use super::db::new_db;
use super::HistoryItem;

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
        let mut db = new_db();
        loop {
            match seq.next_element::<HistoryItem>() {
                Ok(op) => match op {
                    Some(item) => db.add(item),
                    None => break,
                },
                Err(err) => {
                    error!("deserialize failed: {}", err.to_string());
                    return Err(err);
                }
            }
        }
        db.vacuum();
        Ok(db.export_time())
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ReturnType, D::Error>
where
    D: Deserializer<'de>,
{
    let visitor = HistoryVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}
