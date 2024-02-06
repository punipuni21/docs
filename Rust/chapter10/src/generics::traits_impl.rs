use csv::ReaderBuilder;
use std::marker::PhantomData;
use std::path::PathBuf;

#[derive(Defualt)]
pub struct CsvReaderImpl<T> {
    phantom: PhantomData<T>,
}
impl<T> CsvReader<T> for CsvReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        let path_buf = PathBuf::from(file_path);
        let string_data = read_to_string(path_buf);
        let mut reader = ReaderBuilder::new()
            .delimiter(b',')
            .from_reader(string_data.as_bytes());
        let rows = reader.desirialize::<T>();
        let mut result = Vec::<T>::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}

#[derive(Defualt)]
pub struct JsonReaderImpl<T> {
    phantom: PhantomData<T>,
}
impl<T> JsonReader<T> for JsonReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        let path_buf = PathBuf::from(file_path);
        let buf_reader = File::open(path_buf).map(|file| BufReader::new(file))?;
        let result = serde_json::from_reader(buf_reader)?;
        Ok(result)
    }
}
