use crate::association::entities::Product;
use crate::association::traits::{CsvReader, JsonReader};
use crate::association::traits_impl::{CsvReaderImpl, JsonReaderImpl};

#[allow(dead_code)]
pub fn use_association_method() {
    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.csv");
    let json_path = concat!(env!("CARGO_MANIFEST_DIR"), "/resources/products.json");

    let csv_method = CsvReaderImpl::<Product>::default();
    let json_method = JsonReaderImpl::<Product>::default();

    let csv_result = csv_method.read(csv_path).unwrap();
    let json_result = json_method.read(json_path).unwrap();

    println!("<<CSV>>");
    for result in csv_result {
        println!("{:?}", result);
    }
    println!("<<JSON>>");
    for result in json_result {
        println!("{:?}", result);
    }
}
