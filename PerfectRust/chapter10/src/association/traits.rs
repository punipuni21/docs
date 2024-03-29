use anyhow::Result;

pub trait CsvReader {
    type Entity;

    fn read(&self, file_path: &str) -> Result<Vec<Self::Entity>>;
}

pub trait JsonReader {
    type Entity;

    fn read(&self, file_path: &str) -> Result<Vec<Self::Entity>>;
}
