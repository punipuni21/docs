use crate::generics::traits::{CsvReader, JsonReader};
use crate::generics::traits_impl::{CsvReaderImpl, JsonReaderImpl};
use anyhow::Result;
use serde::de::DeserializeOwned;

pub struct ReadService<T> {
    csv_reader: Box<dyn CsvReader<T>>,

    json_reader: Box<dyn JsonReader<T>>,
}

impl<T: DeserializeOwned + 'static> ReadService<T> {
    pub fn new() -> Self {
        Self {
            csv_reader: Box::new(CsvReaderImpl::<T>::new()) as Box<dyn CsvReader<T>>,

            json_reader: Box::new(JsonReaderImpl::<T>::new()) as Box<dyn JsonReader<T>>,
        }
    }

    pub fn csv_reader(&self, file_path: &str) -> Result<Vec<T>> {
        let result = self.csv_reader.read(file_path)?;
        Ok(result)
    }

    pub fn json_reader(&self, file_path: &str) -> Result<Vec<T>> {
        let result = self.json_reader.read(file_path)?;
        Ok(result)
    }
}
