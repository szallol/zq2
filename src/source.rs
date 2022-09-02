use crate::{error::ZqError, table::Table};

type Result<T> = std::result::Result<T, ZqError>;

pub trait Source {
    fn import(&self, source_path: &str) -> Result<Option<Table>>;
}
