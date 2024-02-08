use anyhow::Result;
use serde::de::DeserializeOwned;
use std::marker::PhantomData;

pub trait CsvReader<T>: Send + Sync
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>>;
}

#[derive(Default)]
pub struct CsvReaderImpl<T> {
    phantom: PhantomData<T>,
}

impl<T> CsvReader<T> for CsvReaderImpl<T>
where
    Self: Send + Sync,
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        // implementation
        Ok(Vec::new())
    }
}
