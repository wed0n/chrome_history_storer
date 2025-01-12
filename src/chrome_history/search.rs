use regex::Regex;

use super::{
    batch_insert_item,
    db::{new_db, DB},
    TEMPORARY_DATABASE_FILE_NAME,
};

pub fn search(db: &mut DB, search_pattern: Regex) {
    let mut select = db.select_item();
    let mut new_db = new_db(TEMPORARY_DATABASE_FILE_NAME);
    let _ = batch_insert_item::<()>(&mut new_db, || loop {
        match select() {
            Some(item) => {
                if search_pattern.is_match(&item.title) {
                    return Ok(Some(item));
                }
            }
            None => return Err(()),
        }
    });
}
