use crate::Error;
use std::path::Path;

pub struct Database(sled::Db);

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        Ok(Database(sled::open(path)?))
    }
}
