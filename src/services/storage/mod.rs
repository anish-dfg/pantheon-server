pub mod cache;
pub mod dto;
pub mod entities;
mod errors;
pub mod sql;
#[cfg(test)]
mod tests;
pub mod types;

pub use cache::Cache;
pub use sql::Sql;

use anyhow::Result;

pub struct Storage {
    pub db: Sql,
    pub cache: Cache,
}

impl Storage {
    pub async fn new(database_url: &str, cache_url: &str) -> Result<Self> {
        todo!()
    }
}
