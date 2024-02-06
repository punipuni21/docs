use std::marker::PhantomData;

#[derive(Defualt)]
pub struct CsvReaderImpl<T> {
    phantom: PhantomData<T>,
}
impl<T> CsvReader<T> for CsvReaderImpl<T>
where
    T: DeserializeOwned,
{
    fn read(&self, file_path: &str) -> Result<Vec<T>> {
        todo!()
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
        todo!()
    }
}
